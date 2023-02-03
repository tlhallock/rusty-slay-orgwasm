use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::core::sacrifice::Sacrifice;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;

#[derive(Clone, Debug)]
pub struct ViewHand {
	victim_param: TaskParamName,
}

impl ViewHand {
	pub fn create(victim_param: TaskParamName) -> Box<dyn PlayerTask> {
		Box::new(Self { victim_param }) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for ViewHand {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let victim_index = game.player_param(player_index, &self.victim_param)?;
		game.players[player_index]
			.visible_hands
			.insert(victim_index);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"View someone else's hand".to_owned()
	}
}
