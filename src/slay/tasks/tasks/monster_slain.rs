
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Debug, Clone)]
pub struct MonsterSlainTask {
	pub card_id: ids::CardId,
}

impl PlayerTask for MonsterSlainTask {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.move_card(
			DeckPath::ActiveMonsters,
			DeckPath::SlainMonsters(player_index),
			self.card_id,
		)?;

		if let Some(stack) = game.deck_mut(DeckPath::NextMonsters).maybe_deal() {
			game.deck_mut(DeckPath::ActiveMonsters).add(stack);
		}
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Slay monster card {}.", self.card_id)
	}
}
