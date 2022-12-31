use std::fs::File;
use std::io::Write;

use crate::slay::actions;
use crate::slay::errors::{SlayError, SlayResult};
use crate::slay::game_context;
use crate::slay::ids;
use crate::slay::message;
use crate::slay::specification;
use crate::slay::state;
use crate::slay::state_modifiers;
use crate::slay::strategy;
use crate::slay::tasks;

use rand::seq::SliceRandom;
use rand::Rng;
use state::Card;

fn reset_cards_played_this_turn(game: &mut state::Game) {
	// Could be only the current player...
	game.players.iter_mut().for_each(|p| {
		p.party
			.stacks
			.iter_mut()
			.for_each(|s| s.top.played_this_turn = false)
	});
}

pub fn player_has_won(player: &state::Player) -> bool {
	player.slain_monsters.stacks.len() >= 3 || player.hero_types().len() >= 6
}

pub fn game_is_over(game: &state::Game) -> bool {
	game.players.iter().any(player_has_won)
}

fn use_action_points(context: &mut game_context::GameBookKeeping, game: &mut state::Game) {
	if game.current_player().remaining_action_points > 0 {
		actions::assign_action_choices(context, game);
	}
	reset_cards_played_this_turn(game);
	game.increment();
	game.clear_expired_modifiers();
	game.current_player_mut().remaining_action_points = 3;
	actions::assign_action_choices(context, game);
}

fn check_for_expired_modifiers(game: &mut state::Game) {
	for _player in game.players.iter_mut() {}
	todo!()
}

pub fn initialize_game(context: &mut game_context::GameBookKeeping, game: &mut state::Game) {
	let (draw_capacity, leaders_capacity, monsters_capacity) = (101, 10, 20);
	let mut draw = Vec::with_capacity(draw_capacity);
	let mut leaders = Vec::with_capacity(leaders_capacity);
	let mut monsters = Vec::with_capacity(monsters_capacity);
	specification::get_card_specs().iter().for_each(|spec| {
		for _ in 0..spec.repeat {
			let stack = state::Stack::new(Card::new(context.id_generator.generate(), spec.to_owned()));
			match spec.card_type {
				specification::CardType::PartyLeader(_) => leaders.push(stack),
				specification::CardType::Monster => monsters.push(stack),
				_ => draw.push(stack),
			};
		}
	});
	if draw_capacity != draw.len() {
		println!("Draw's capacity should be {}", draw.len())
	}
	if leaders_capacity != draw.len() {
		println!("Leader's capacity should be {}", leaders.len())
	}
	if monsters_capacity != draw.len() {
		println!("Monster's capacity should be {}", monsters.len())
	}

	[&mut draw, &mut leaders, &mut monsters]
		.iter_mut()
		.for_each(|deck| deck.shuffle(&mut context.rng));

	game.draw.stacks.extend(draw);
	game.next_monsters.stacks.extend(monsters);
	game.leaders.stacks.extend(leaders);

	state_modifiers::transfer_upto_n(3, &mut game.next_monsters, &mut game.monsters);

	for player_index in 0..4 {
		let mut player = state::Player::new(
			&mut context.id_generator,
			format!("Unnamed bot {}", player_index + 1),
			player_index,
			game.leaders.deal().top,
		);
		state_modifiers::transfer_upto_n(5, &mut game.draw, &mut player.hand);
		game.players.push(player);
	}

	// initialize the first first random player
	game.set_active_player(context.rng.gen_range(0..game.number_of_players()));

	game.current_player_mut().remaining_action_points = 3;
	actions::assign_action_choices(context, game);
}

pub enum AdvanceGameResult {
	Complete,
	WaitingForPlayers,
	// ContinueAdvancing,
}

fn waiting_for_players(game: &state::Game) -> bool {
	game.players.iter().any(|p| p.choices.is_some())
}

pub fn advance_game(
	context: &mut game_context::GameBookKeeping, game: &mut state::Game,
) -> SlayResult<AdvanceGameResult> {
	// TODO: We never check if the choices have expired!
	for _ in 0..10000 {
		if game_is_over(game) {
			return Ok(AdvanceGameResult::Complete);
		}

		if let Some(mut showdown) = game.showdown.take_complete() {
			showdown.finish(context, game);
			continue;
		}

		let mut waiting_for_somebody = false;
		let number_of_players = game.number_of_players();
		for player_index in 0..number_of_players {
			match tasks::continue_tasks(context, game, player_index)? {
				tasks::TaskProgressResult::TaskComplete => {}
				tasks::TaskProgressResult::ChoicesAssigned => {
					waiting_for_somebody = true;
				}
			}
		}
		if waiting_for_somebody {
			return Ok(AdvanceGameResult::WaitingForPlayers);
		}

		use_action_points(context, game);
		return Ok(AdvanceGameResult::WaitingForPlayers);
	}
	unreachable!("Infinite loop?");
}

pub fn make_selection(
	context: &mut game_context::GameBookKeeping, game: &mut state::Game, player_id: ids::ElementId,
	choice_id: ids::ElementId,
) -> SlayResult<()> {
	let player_index = game
		.player_index(player_id)
		.ok_or_else(|| SlayError::new("Player not found."))?;

	// TODO: this doesn't copy a Choices on the stack does it?
	let choices = Some(
		game.players[player_index]
			.choices
			.take()
			.ok_or_else(|| SlayError::new("No active choices."))?,
	);

	let mut binding = choices.unwrap();
	let choice = binding
		// .ok_or_else(|| SlayError::new("No active choices."))?
		.options
		.iter_mut()
		.find(|c| c.get_choice_information().get_id() == choice_id)
		.ok_or_else(|| SlayError::new("Choice not found."))?;

	context.emit(&message::Notification {
		message_text: format!(
			"Player {} chose {}",
			player_index,
			choice.get_choice_information().display.label
		),
	});
	choice.select(context, game)?;
	Ok(())
}

pub fn game_loop() -> SlayResult<()> {
	let context = &mut game_context::GameBookKeeping::new();
	let game = &mut state::Game::new(context);

	initialize_game(context, game);

	let iteration = 0;
	'turns: loop {
		if game.get_turn().over_the_limit() {
			return Err(SlayError::new("Hit maximum iterations"));
		}

		{
			let mut file = File::create(format!("output/turn_{}.txt", iteration)).expect("msg");
			file
				.write_all(format!("{:#?}", game).as_bytes())
				.expect("msg");
		}
		// println!("{}", game);

		// let serialized = serde_json::to_string(game).unwrap();
		// println!("{}", serialized);

		// let perspective = GamePerspective::from(game, 1);
		// let html = view::show_perspective(perspective);

		let (player_id, choice_id) = strategy::pick_a_random_choice(context, game)?;
		make_selection(context, game, player_id, choice_id)?;
		'advancing: loop {
			match advance_game(context, game)? {
				AdvanceGameResult::Complete => return Ok(()),
				AdvanceGameResult::WaitingForPlayers => continue 'turns,
				// AdvanceGameResult::ContinueAdvancing => continue 'advancing,
			}
		}
	}
}
