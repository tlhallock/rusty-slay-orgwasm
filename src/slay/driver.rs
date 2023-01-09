use log::LevelFilter;

use crate::slay::actions;
use crate::slay::errors::{SlayError, SlayResult};
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::message::Notification;
use crate::slay::state::game::Game;
use crate::slay::state::player::Player;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::tasks::TaskProgressResult;
use crate::slay::{strategy, tasks};

use std::io::BufWriter;

use simple_logging;

use super::state::initialize;

pub fn player_has_won(player: &Player) -> bool {
	player.slain_monsters.num_top_cards() >= 3 || player.hero_types().len() >= 6
}

pub fn game_is_over(game: &Game) -> bool {
	game.players.iter().any(player_has_won)
}

fn use_action_points(context: &mut GameBookKeeping, game: &mut Game) {
	if game.current_player().get_remaining_action_points() > 0 {
		log::info!("Assigning action points");
		actions::assign_action_choices(context, game);
		return;
	}
	game.current_player_mut().turn_end();
	game.increment();
	game.clear_expired_modifiers();
	game.current_player_mut().turn_begin();
	actions::assign_action_choices(context, game);
}

fn check_for_expired_modifiers(game: &mut Game) {
	for _player in game.players.iter_mut() {}
	todo!()
}


pub enum AdvanceGameResult {
	GameOver,
	WaitingForPlayers,
	// ContinueAdvancing,
}

fn waiting_for_players(game: &Game) -> bool {
	game.players.iter().any(|p| p.choices.is_some())
}

fn run_tasks(context: &mut GameBookKeeping, game: &mut Game) -> SlayResult<TaskProgressResult> {
	let mut result = TaskProgressResult::NothingDone;
	let number_of_players = game.number_of_players();
	for player_index in 0..number_of_players {
		match tasks::continue_tasks(context, game, player_index)? {
			TaskProgressResult::NothingDone => {}
			TaskProgressResult::TaskComplete | TaskProgressResult::ProgressMade => {
				result = TaskProgressResult::ProgressMade;
			}
		}
	}
	Ok(result)
}

pub fn advance_game(
	context: &mut GameBookKeeping, game: &mut Game,
) -> SlayResult<AdvanceGameResult> {
	// TODO: We never check if the choices have expired!

	let mut iteration = 0;
	loop {
		iteration += 1;
		if iteration > 10000 {
			unreachable!();
		}

		if game_is_over(game) {
			return Ok(AdvanceGameResult::GameOver);
		}
		if let Some(mut showdown) = game.showdown.take_complete() {
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
	game: &mut Game, player_index: ids::PlayerIndex, choice_id: ids::ElementId,
	notify: &mut dyn FnMut(Notification),
) -> SlayResult<()> {
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
		.find(|c| c.id == choice_id)
		.ok_or_else(|| SlayError::new("Choice not found."))?;

	/*context.emit*/
	notify(Notification {
		message_text: format!("Player {} chose {}", player_index, choice.display.label),
	});
	choice.select(game, player_index)?;
	Ok(())
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
			return Ok(());
			// return Err(SlayError::new("Hit maximum iterations"));
		}

		{
			// log::info!("Writing iteration {} to file.", iteration);
			// let write_file = File::create(
			// 	format!("./output/iteration_{:04}.txt", iteration))
			// 	.unwrap();
			// let mut writer = BufWriter::new(&write_file);
			let mut writer = BufWriter::new(Vec::new());
			game
				.summarize(&mut writer, 0)
				.expect("Error writing to file");
			let bytes = writer.into_inner().expect("whoops");
			let string = String::from_utf8(bytes).expect("error logging state");
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

		let (player_id, choice_id) = strategy::pick_a_random_choice(context, game)?;
		make_selection(game, player_id, choice_id, &mut |notification| {
			log::info!("Notification: '{}'", notification.message_text);
		})?;
		match advance_game(context, game)? {
			AdvanceGameResult::GameOver => return Ok(()),
			AdvanceGameResult::WaitingForPlayers => continue 'turns,
		}
	}
}

/*
Tests
	Place a hero card without challenging.
	replentishing the draw pile






 */
