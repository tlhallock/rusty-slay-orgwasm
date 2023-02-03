use crate::slay::choices::Choices;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::notification::Notification;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct ChooseTask {
	choices: Option<Choices>,
}

impl ChooseTask {
	pub fn new(choices: Choices) -> Self {
		Self {
			choices: Some(choices),
		}
	}
	pub fn create(choices: Choices) -> Box<dyn PlayerTask> {
		Box::new(Self::new(choices))
	}
}

impl PlayerTask for ChooseTask {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		if let Some(choices) = self.choices.as_ref() { context.emit(&Notification::PlayerIsChoosing(
				player_index,
				choices.choices_type.to_owned(),
			)) }
		game.players[player_index].choices_ = self.choices.take();
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		"Choose something".to_string()
	}
}
