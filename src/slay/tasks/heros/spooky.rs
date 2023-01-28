use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::core::sacrifice::Sacrifice;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct Spooky {}

impl Spooky {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for Spooky {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		for victim_index in 0..game.number_of_players() {
			if player_index == victim_index {
				continue;
			}
			game.players[victim_index]
				.tasks
				.prepend(Sacrifice::create(1));
		}
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"do spooky".to_owned()
	}
}
