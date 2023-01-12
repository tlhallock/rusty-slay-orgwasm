use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct CardUsedTask {
	card_id: ids::CardId,
}

impl CardUsedTask {
	pub fn new(_player_index: ids::PlayerIndex, card_id: ids::CardId) -> Self {
		Self { card_id }
	}
}
impl PlayerTask for CardUsedTask {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[player_index].set_card_played(self.card_id);
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("Marking {} as used", self.card_id)
	}
}
