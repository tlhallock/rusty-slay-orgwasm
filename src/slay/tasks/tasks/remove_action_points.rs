use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct RemoveActionPointsTask {
	amount: u32,
}

impl RemoveActionPointsTask {
	pub fn new(amount: u32) -> Self {
		Self { amount }
	}
}
impl PlayerTask for RemoveActionPointsTask {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[player_index].action_points_used(self.amount);
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("Deducting {} action points.", self.amount)
	}
}
