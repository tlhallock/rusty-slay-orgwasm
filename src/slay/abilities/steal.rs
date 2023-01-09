use super::params::ChooseCardFromPlayerParameterTask;
use crate::slay::abilities::params::ChoosePlayerParameterTask;
use crate::slay::abilities::params::ClearParamsTask;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskParamName;
use crate::slay::tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct StealTask {}

impl StealTask {
	pub fn new() -> Self {
		Self {}
	}

	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {})
	}
}

impl PlayerTask for StealTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, thief_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[thief_index].tasks.prepend_from(&mut vec![
			ChoosePlayerParameterTask::exclude_self(TaskParamName::PlayerToStealFrom, "to steal from"),
			// This one coulds just be a method call, all it does is assign a new task...
			// (Kinda like the steal action...)
			// I guess it should just be renamed to 'choose card from player's party' or smh...
			// Could be from DeckPath...
			ChooseCardFromPlayerParameterTask::from_party(
				TaskParamName::PlayerToStealFrom,
				TaskParamName::CardToSteal,
				"Which hero card would you like to steal?",
			),
			StealCardFromTask::create(TaskParamName::PlayerToStealFrom, TaskParamName::CardToSteal),
			ClearParamsTask::create(),
		]);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Preparing to steal a card")
	}
}

#[derive(Clone, Debug)]
pub struct StealCardFromTask {
	victim_param: TaskParamName,
	card_param: TaskParamName,
}

impl StealCardFromTask {
	pub fn create(victim_param: TaskParamName, card_param: TaskParamName) -> Box<dyn PlayerTask> {
		Box::new(Self {
			victim_param,
			card_param,
		}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for StealCardFromTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, stealer_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let victim_player_index = game.player_param(stealer_index, &self.victim_param)?;
		let card_to_steal = game.card_param(stealer_index, &self.card_param)?;
		if card_to_steal.is_none() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let card_to_steal = card_to_steal.unwrap();
		let stolen_stack = game.players[victim_player_index]
			.party
			.take_card(card_to_steal)?;
		// TODO: Check if we are actually supposed to do something else due to buffs...
		game.players[stealer_index].party.add(stolen_stack);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Player is stealing a card from a specific individual.".to_string()
	}
}
