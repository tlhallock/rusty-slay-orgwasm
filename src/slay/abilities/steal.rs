



use crate::slay::choices::DisplayPath;


use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskParamName;
use crate::slay::tasks::TaskProgressResult;

use super::params::CardChoiceInformation;
use super::params::ChooseCardParameterTask;
use super::params::ChoosePlayerParameterTask;
use super::params::ClearParamsTask;

#[derive(Clone, Debug)]
pub struct StealTask {
	pub thief_index: ids::PlayerIndex,
}

impl PlayerTask for StealTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, _player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[self.thief_index].tasks.prepend_from(&mut vec![
			Box::new(ChoosePlayerParameterTask {
				param_name: TaskParamName::PlayerToStealFrom,
				instructions: "to steal from".to_owned(),
			}) as Box<dyn PlayerTask>,
			// This one coulds just be a method call, all it does is assign a new task...
			// (Kinda like the steal action...)
			// I guess it should just be renamed to 'choose card from player's party' or smh...
			// Could be from DeckPath...
			Box::new(StealFromTask {
				victim_param: TaskParamName::PlayerToStealFrom,
			}) as Box<dyn PlayerTask>,
			Box::new(StealCardFromTask {
				victim_param: TaskParamName::PlayerToStealFrom,
				card_param: TaskParamName::CardToSteal,
			}) as Box<dyn PlayerTask>,
			Box::new(ClearParamsTask {}) as Box<dyn PlayerTask>,
		]);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Player {} is preparing to steal a card.", self.thief_index)
	}
}

#[derive(Clone, Debug)]
pub struct StealFromTask {
	victim_param: TaskParamName,
}

impl PlayerTask for StealFromTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, stealer_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		if let Some(victim_player_index) = game.players[stealer_index]
			.tasks
			.get_player_value(&self.victim_param)
		{
			let card_choices: Vec<CardChoiceInformation> = game.players[stealer_index]
				.party
				.iter()
				.map(|stack| CardChoiceInformation {
					card_id: stack.top.id,
					display_path: DisplayPath::CardIn(DeckPath::Party(victim_player_index), stack.top.id),
					card_label: stack.top.spec.label.to_owned(),
				})
				.collect();

			if card_choices.is_empty() {
				game.players[stealer_index]
					.tasks
					.set_card_value(TaskParamName::CardToSteal, None)?;
				return Ok(TaskProgressResult::TaskComplete);
			}

			game.players[stealer_index]
				.tasks
				.prepend(Box::new(ChooseCardParameterTask {
					param_name: TaskParamName::CardToSteal,
					instructions: "Choose which card to steal".to_string(),
					card_choices,
				}));
			Ok(TaskProgressResult::TaskComplete)
		} else {
			Ok(TaskProgressResult::NothingDone)
		}
	}

	fn label(&self) -> String {
		format!("Player is stealing a card from a specific individual.",)
	}
}

#[derive(Clone, Debug)]
pub struct StealCardFromTask {
	victim_param: TaskParamName,
	card_param: TaskParamName,
}

impl PlayerTask for StealCardFromTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, stealer_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let victim_player_index = game.players[stealer_index]
			.tasks
			.get_player_value(&self.victim_param)
			.ok_or_else(|| SlayError::new("Required a player to already be chosen."))?;

		let card_to_steal = game.players[stealer_index]
			.tasks
			.get_card_value(&self.card_param)
			.ok_or_else(|| SlayError::new("Required a card to already be chosen."))?;

		if card_to_steal.is_none() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let card_to_steal = card_to_steal.unwrap();

		let stolen_stack = game.players[victim_player_index]
			.party
			.take_card(card_to_steal)?;
		// TODO: Check if we are actually supposed to do something else due to buffs...

		// This is supposed to be for destroying a stack...
		// game.discard.extend(stolen_stack.modifiers.drain(..));
		game.players[stealer_index].party.add(stolen_stack);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Player is stealing a card from a specific individual.",)
	}
}
