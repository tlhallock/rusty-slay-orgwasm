

use crate::slay::tasks::PlayerTask;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::state::Game;
use crate::slay::choices::Choices;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;







pub enum TaskParameterType {
  Player,
  Card,
  OneOf,

}

pub struct ChoosePlayerTask {
  // pub parameter_type: TaskParameterType,
  pub key: String,
  pub instructions: String,
  pub player_index: usize,
}

impl PlayerTask for ChoosePlayerTask {
	fn make_progress(
		&mut self,
    context: &mut GameBookKeeping,
    game: &mut Game,
	) -> SlayResult<TaskProgressResult> {
    game.players[player_index].choices = Choices {
      options: 0..game.number_of_players()
        .filter(|index| index != self.player_index)
        .map(
          |victim_index| TasksChoice::prepend(
            
          )
        )
        .collect(),
      default_choice: None,
      timeline: deadlines::get_discard_deadline(),
      instructions: self.instructions.to_owned(),
      };
    
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!(
			"Player {} places the item {}",
			self.player_index, self.card_id
		)
	}
}

pub struct SetPlayerTask {
  pub key: String,
  pub instructions: String,
  pub player_index: usize,
}

impl PlayerTask for SetPlayerTask {
	fn make_progress(
		&mut self,
    context: &mut GameBookKeeping,
    game: &mut Game,
	) -> SlayResult<TaskProgressResult> {
    
    
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!(
			"Player {} places the item {}",
			self.player_index, self.card_id
		)
	}
}
