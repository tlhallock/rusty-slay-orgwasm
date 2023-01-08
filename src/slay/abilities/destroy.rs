use crate::slay::abilities::params::ChooseCardFromPlayerParameterTask;
use crate::slay::abilities::params::ChoosePlayerParameterTask;
use crate::slay::abilities::params::ClearParamsTask;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Stack;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskParamName;
use crate::slay::tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct DestroyTask {
	pub thief_index: ids::PlayerIndex,
}

impl PlayerTask for DestroyTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, _player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[self.thief_index].tasks.prepend_from(&mut vec![
			ChoosePlayerParameterTask::create(TaskParamName::PlayerToDestroy, "to destroy a hero card"),
			ChooseCardFromPlayerParameterTask::from_party(
				TaskParamName::PlayerToDestroy,
				TaskParamName::CardToDestroy,
				"Which hero card would you like to destroy?",
			),
			DestroyCardTask::create(
				TaskParamName::PlayerToDestroy,
				TaskParamName::CardToDestroy,
				DestroyModifiersDestination::Discard,
			),
			ClearParamsTask::create(),
		]);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!(
			"Player {} is preparing to destroy a card.",
			self.thief_index
		)
	}
}

#[derive(Clone, Debug)]
pub enum DestroyModifiersDestination {
	Myself,
	Discard,
}

#[derive(Clone, Debug)]
pub struct DestroyCardTask {
	victim_param: TaskParamName,
	card_param: TaskParamName,
	destination: DestroyModifiersDestination,
}

impl DestroyCardTask {
	pub fn create(
		victim_param: TaskParamName, card_param: TaskParamName,
		destination: DestroyModifiersDestination,
	) -> Box<dyn PlayerTask> {
		Box::new(Self {
			victim_param,
			card_param,
			destination,
		}) as Box<dyn PlayerTask>
	}

	fn get_destination(&self, player_index: ids::PlayerIndex) -> DeckPath {
		match self.destination {
			DestroyModifiersDestination::Myself => DeckPath::Hand(player_index),
			DestroyModifiersDestination::Discard => DeckPath::Discard,
		}
	}
}

impl PlayerTask for DestroyCardTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, stealer_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let victim_player_index = game.player_param(stealer_index, &self.victim_param)?;
		let card_id = game.card_param(stealer_index, &self.card_param)?;
		if card_id.is_none() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let card_to_steal = card_id.unwrap();

		let mut stack = game.players[victim_player_index]
			.party
			.take_card(card_to_steal)?;
		game
			.deck_mut(self.get_destination(stealer_index))
			.extend(stack.modifiers.drain(..).map(|card| Stack::new(card)));
		game.deck_mut(DeckPath::Discard).add(stack);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Player is stealing a card from a specific individual.".to_string()
	}
}
