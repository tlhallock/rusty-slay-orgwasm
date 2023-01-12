





use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;


use crate::slay::showdown::roll_modification::ModificationPath;
use crate::slay::showdown::roll_modification::RollModification;



use crate::slay::state::game::Game;

use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;


#[derive(Debug, Clone)]
pub struct ModifyRollTask {
	modification: RollModification,
	modification_path: ModificationPath,
}
impl ModifyRollTask {
	pub fn new(modification: RollModification, path: ModificationPath) -> Self {
		Self {
			modification,
			modification_path: path,
		}
	}
}

impl PlayerTask for ModifyRollTask {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		_player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let modification = self.modification.to_owned();

		game
			.showdown
			.add_modification(self.modification_path, modification)?;
		let modification_task = game.showdown.get_modification_task(context, game);
		modification_task.apply(context, game);
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!(
			"Player modifying {:?} with {:?}",
			self.modification_path, self.modification
		)
	}
}
