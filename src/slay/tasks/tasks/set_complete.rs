use crate::slay::{
	choices::{ChoiceDisplay, ChoiceDisplayType, TasksChoice},
	errors::SlayResult,
	game_context::GameBookKeeping,
	ids,
	showdown::completion::Completion,
	state::game::Game,
	tasks::player_tasks::{PlayerTask, TaskProgressResult},
};

#[derive(Debug, Clone)]
pub struct SetCompleteTask {
	persist: Completion,
}

impl SetCompleteTask {
	pub fn new(persist: Completion) -> Self {
		Self { persist }
	}
}

impl PlayerTask for SetCompleteTask {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game
			.showdown
			.set_player_completion(player_index, self.persist)?;
		game.players[player_index].choices = None;
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("Setting completion to {:?}", self.persist)
	}
}

fn create_set_complete_choice(
	id: ids::ChoiceId,
	persist: Completion,
	label: String,
) -> TasksChoice {
	TasksChoice::new(
		id,
		ChoiceDisplay {
			label,
			display_type: ChoiceDisplayType::SetCompletion(persist),
		},
		vec![Box::new(SetCompleteTask::new(persist)) as Box<dyn PlayerTask>],
	)
}

pub fn create_set_completion_done(id: ids::ChoiceId) -> TasksChoice {
	create_set_complete_choice(id, Completion::AllDone, "Do nothing.".to_string())
}

pub fn create_set_completion_until_modification(id: ids::ChoiceId) -> TasksChoice {
	create_set_complete_choice(
		id,
		Completion::DoneUntilModification,
		"Don't modify this roll unless someone else does.".to_string(),
	)
}
