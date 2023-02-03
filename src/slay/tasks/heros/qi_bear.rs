use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::core::destroy::DestroyTask;
use crate::slay::tasks::core::discard::Discard;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::tasks::params::ClearParamsTask;

#[derive(Clone, Debug)]
pub struct QiBear {
	num_remaining: u32,
}

impl QiBear {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self { num_remaining: 3 }) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for QiBear {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		if self.num_remaining == 0 {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let default_choice = context.id_generator.generate();

		game.players[player_index].choose(Choices {
			choices_type: ChoicesType::ContinueDiscardingAndDestroying(self.num_remaining),
			default_choice: None,
			timeline: deadlines::get_refactor_me_deadline(),
			options: vec![
				TasksChoice::prepend(
					context.id_generator.generate(),
					// The argument here is just so the user knows why they are being asked again...
					Choice::ContinueDiscardingAndDestroying,
					ChoiceDisplayType::Yes,
					vec![
						Discard::create(1),
						DestroyTask::create(),
						ClearParamsTask::create(),
					],
				),
				TasksChoice::prepend(
					default_choice,
					Choice::QuitAction,
					ChoiceDisplayType::No,
					vec![
						// TODO: this is a hassle: it is going to ask three times no
						// matter what.
					],
				),
			],
		});
		// we could have just added a new one to the queue...
		if self.num_remaining == 1 {
			Ok(TaskProgressResult::TaskComplete)
		} else {
			self.num_remaining -= 1;
			Ok(TaskProgressResult::ProgressMade)
		}
	}

	fn label(&self) -> String {
		format!("do qi bear, num remaining = {}", self.num_remaining)
	}
}
