use crate::slay::choices::{Choice, ChoiceInformation};
use crate::slay::game_context::GameBookKeeping;
use crate::slay::state::Game;

use super::tasks::PlayerTask;

#[derive(Debug, Clone)]
pub struct TasksChoice {
    choice_information: ChoiceInformation,
    tasks: Vec<Box<dyn PlayerTask>>,
}

impl TasksChoice {
    pub fn new(choice_information: ChoiceInformation, tasks: Vec<Box<dyn PlayerTask>>) -> Self {
        Self {
            choice_information,
            tasks,
        }
    }
}

impl Choice for TasksChoice {
    fn select(
        &mut self,
        context: &mut GameBookKeeping,
        game: &mut Game,
    ) -> super::errors::SlayResult<()> {
        let player_index = self.choice_information.player_index();
        game.players[player_index].tasks.take_from(&mut self.tasks);
        Ok(())
    }

    fn get_choice_information(&self) -> &ChoiceInformation {
        &self.choice_information
    }
}
