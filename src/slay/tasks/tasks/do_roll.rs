





use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;


use crate::slay::showdown::completion::CompletionTracker;



use crate::slay::showdown::current_showdown::ShowDown;



use crate::slay::showdown::roll_state::RollState;


use crate::slay::state::game::Game;


use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct DoRollTask {
	roll: Option<RollState>,
}

impl DoRollTask {
	pub fn new(roll: RollState) -> Self {
		Self { roll: Some(roll) }
	}
}
impl PlayerTask for DoRollTask {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		_player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		if let Some(mut roll) = self.roll.take() {
			roll.completion_tracker = Some(CompletionTracker::new(
				game.number_of_players(),
				deadlines::get_roll_deadline(),
			));
			roll.assign_all_choices(context, game);
			game.showdown.roll(roll);
			Ok(TaskProgressResult::TaskComplete)
		} else {
			Err(SlayError::new("Can only perform a choice once..."))
		}
	}
	fn label(&self) -> String {
		format!("Doing a roll task for {:?}", self.roll)
	}
}
