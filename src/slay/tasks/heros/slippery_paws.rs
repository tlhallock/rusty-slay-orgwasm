use std::collections::HashSet;

use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::core::discard::Discard;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;

#[derive(Clone, Debug)]
pub struct SlipperyPaws {}

impl SlipperyPaws {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for SlipperyPaws {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let first_card =
			game.card_param(player_index, &TaskParamName::SlipperyPawsVictimPulledCard1)?;
		let second_card =
			game.card_param(player_index, &TaskParamName::SlipperyPawsVictimPulledCard2)?;
		if first_card.is_none() || second_card.is_none() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let mut include = HashSet::new();
		include.insert(first_card.unwrap());
		include.insert(second_card.unwrap());
		// include = std::iter::once(first_card.unwrap()).chain(std::iter::once(second_card.unwrap())).collect();
		// let exclude = game
		// 	.deck(DeckPath::Hand(player_index))
		// 	.other_cards(&include);
		game.players[player_index]
			.tasks
			.prepend(Box::new(Discard::discard_one_of(include)));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"do slippery paws".to_owned()
	}
}
