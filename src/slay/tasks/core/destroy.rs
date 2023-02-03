use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Stack;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::params::ChooseCardFromPlayerParameterTask;
use crate::slay::tasks::tasks::params::ChoosePlayerParameterTask;
use crate::slay::tasks::tasks::params::ClearParamsTask;

#[derive(Clone, Debug)]
pub struct DestroyTask {}

impl DestroyTask {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {})
	}
}

impl PlayerTask for DestroyTask {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		thief_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[thief_index].tasks.prepend_from(&mut vec![
			ChoosePlayerParameterTask::exclude_self(TaskParamName::PlayerToDestroy),
			ChooseCardFromPlayerParameterTask::from_party(
				TaskParamName::PlayerToDestroy,
				TaskParamName::CardToDestroy,
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
		"Player is preparing to destroy a card.".to_string()
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
		victim_param: TaskParamName,
		card_param: TaskParamName,
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
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		stealer_index: ids::PlayerIndex,
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
			.extend(stack.modifiers.drain(..).map(Stack::new));
		game.deck_mut(DeckPath::Discard).add(stack);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Player is stealing a card from a specific individual.".to_string()
	}
}
