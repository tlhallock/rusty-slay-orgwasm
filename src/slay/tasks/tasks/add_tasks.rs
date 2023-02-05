use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

// Emit logs like "Waiting for challenges..."

#[derive(Debug, Clone)]
pub struct AddTasks {
	pub tasks: Vec<Box<dyn PlayerTask>>,
}

impl AddTasks {
	pub fn create(tasks: Vec<Box<dyn PlayerTask>>) -> Box<dyn PlayerTask> {
		Box::new(Self { tasks })
	}
}

impl PlayerTask for AddTasks {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[player_index]
			.tasks
			.prepend_from(&mut self.tasks);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Adding some tasks".to_owned()
	}
}
