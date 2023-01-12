







use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;












use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;


use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Debug, Clone)]
pub struct MoveCardTask {
	// Now that I have a card path, I could just use that...
	pub source: DeckPath,
	pub destination: DeckPath,
	pub card_id: ids::CardId,
	// Could have a replentish here?
}

impl PlayerTask for MoveCardTask {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		_player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.move_card(self.source, self.destination, self.card_id)?;
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!(
			"Moving {} from {:?} to {:?}",
			self.card_id, self.source, self.destination
		)
	}
}
