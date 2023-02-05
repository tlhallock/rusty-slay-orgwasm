use crate::slay::actions::list_actions;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::notification::Notification;
use crate::slay::state::game::Game;
use crate::slay::state::initialize;
use crate::slay::state::player::Player;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::strategy;
use crate::slay::tasks::player_tasks;
use crate::slay::tasks::player_tasks::TaskProgressResult;

use std::collections::HashSet;
use std::io::BufWriter;

use log::LevelFilter;
use simple_logging;

pub fn player_has_won(player: &Player) -> bool {
	let hero_types = &mut HashSet::new();
	player.collect_hero_types(hero_types);

	let num_monsters = player.slain_monsters.num_top_cards();
	if num_monsters >= 3 {
		log::info!(
			"Player {} has {} slain monsters.",
			player.player_index,
			num_monsters
		);
		return false;
		// return true;
	}
	if hero_types.len() >= 6 {
		log::info!(
			"Player {} has {} different hero types.",
			player.player_index,
			hero_types.len()
		);
		return false;
		// return true;
	}
	false
}

pub fn game_is_over(game: &Game) -> Option<ids::PlayerIndex> {
	for player in game.players.iter() {
		if player_has_won(player) {
			return Some(player.player_index);
		}
	}
	None
}

fn use_action_points(context: &mut GameBookKeeping, game: &mut Game) {
	if game.current_player().get_remaining_action_points() > 0 {
		log::info!("Assigning action points");
		list_actions::assign_action_choices(context, game);
		return;
	}
	game.current_player_mut().turn_end();
	game.increment();
	context.emit(&Notification::PlayersTurn(game.active_player_index()));
	game.clear_expired_modifiers();
	game.current_player_mut().turn_begin();
	list_actions::assign_action_choices(context, game);
}

pub enum AdvanceGameResult {
	GameOver,
	WaitingForPlayers,
	// ContinueAdvancing,
}

fn waiting_for_players(game: &Game) -> bool {
	game.players.iter().any(|p| p.has_choices())
}

fn run_tasks(context: &mut GameBookKeeping, game: &mut Game) -> SlayResult<TaskProgressResult> {
	let mut result = TaskProgressResult::NothingDone;
	let number_of_players = game.number_of_players();
	for player_index in 0..number_of_players {
		match player_tasks::continue_tasks(context, game, player_index)? {
			TaskProgressResult::NothingDone => {}
			TaskProgressResult::TaskComplete | TaskProgressResult::ProgressMade => {
				result = TaskProgressResult::ProgressMade;
			}
		}
	}
	Ok(result)
}

pub fn advance_game(
	context: &mut GameBookKeeping,
	game: &mut Game,
) -> SlayResult<AdvanceGameResult> {
	let mut iteration = 0;
	loop {
		iteration += 1;
		if iteration > 10000 {
			unreachable!();
		}

		if let Some(winner_index) = game_is_over(game) {
			context.emit(&Notification::PlayerWon(winner_index));
			return Ok(AdvanceGameResult::GameOver);
		}
		if let Some(mut showdown) = game.showdown.take_complete() {
			game.clear_choices();
			showdown.finish(context, game);
			continue;
		}

		match run_tasks(context, game)? {
			TaskProgressResult::NothingDone => break,
			TaskProgressResult::ProgressMade | TaskProgressResult::TaskComplete => continue,
		}
	}

	if !waiting_for_players(game) {
		if !game.showdown.is_empty() {
			unreachable!();
		}
		use_action_points(context, game);
	}
	Ok(AdvanceGameResult::WaitingForPlayers)
}

pub fn make_selection(
	game: &mut Game,
	player_index: ids::PlayerIndex,
	choice_id: ids::ElementId,
	notify: &mut dyn FnMut(Notification),
) -> SlayResult<()> {
	// TODO: this doesn't copy a Choices on the stack does it?
	let choices = Some(
		game.players[player_index]
			.choices_
			.take()
			.ok_or_else(|| SlayError::new("No active choices."))?,
	);

	let mut binding = choices.unwrap();
	let choice = binding
		// .ok_or_else(|| SlayError::new("No active choices."))?
		.options
		.iter_mut()
		.find(|c| c.id == choice_id)
		.ok_or_else(|| SlayError::new("Choice not found."))?;

	/*context.emit*/
	notify(Notification::PlayerChose(
		player_index,
		choice.choice.to_owned(),
	));
	choice.select(game, player_index)?;
	Ok(())
}

fn game_to_string(game: &Game) -> String {
	let mut writer = BufWriter::new(Vec::new());
	game
		.summarize(&mut writer, 0)
		.expect("Error writing to file");
	let bytes = writer.into_inner().expect("whoops");

	String::from_utf8(bytes).expect("error logging state")
}

pub fn game_loop() -> SlayResult<()> {
	simple_logging::log_to_file("output/log.txt", LevelFilter::Info).expect("Unable to log.");

	let context = &mut GameBookKeeping::new();
	let game = &mut Game::new();

	initialize::initialize_game(context, game);

	let mut iteration = 0;
	'turns: loop {
		iteration += 1;

		if game.get_turn().over_the_limit() {
			log::info!("Reached iteration cap.");
			return Ok(());
			// return Err(SlayError::new("Hit maximum iterations"));
		}

		{
			// log::info!("Writing iteration {} to file.", iteration);
			// let write_file = File::create(
			// 	format!("./output/iteration_{:04}.txt", iteration))
			// 	.unwrap();
			// let mut writer = BufWriter::new(&write_file);

			let string = game_to_string(game);
			log::info!("iteration {:04} information:\n{}", iteration, string);
		}

		// flush!(writer);

		// {
		// 	let mut file = File::create(format!("output/turn_{}.txt", iteration)).expect("msg");
		// 	file
		// 		.write_all(format!("{:#?}", game).as_bytes())
		// 		.expect("msg");
		// }
		// println!("{}", game);

		// let serialized = serde_json::to_string(game).unwrap();
		// println!("{}", serialized);

		// let perspective = GamePerspective::from(game, 1);
		// let html = view::show_perspective(perspective);

		let (player_index, choice_id) = strategy::pick_a_random_choice(context, game)?;
		let statics = &game.to_statics(player_index);
		make_selection(game, player_index, choice_id, &mut |notification| {
			log::info!(
				"Notification: '{}'",
				notification.get_description(statics, player_index)
			);
		})?;
		match advance_game(context, game)? {
			AdvanceGameResult::GameOver => {
				let string = game_to_string(game);
				println!("Ending state: {}", string);
				return Ok(());
			}
			AdvanceGameResult::WaitingForPlayers => continue 'turns,
		}
	}
}

/*
Tests
	Place a hero card without challenging.
	replentishing the draw pile









 */
