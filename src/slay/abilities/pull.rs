
use rand::Rng;

use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceInformation;
use crate::slay::choices::ChoiceLocator;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskParamName;
use crate::slay::tasks::TaskProgressResult;
use crate::slay::state::deck::DeckPath;




#[derive(Clone, Debug)]
pub struct PullFromTask {
  pub pulled_index_param_name: TaskParamName,
}

impl PlayerTask for PullFromTask {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game,
    puller_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		if let Some(victim_index) = game.players[puller_index].tasks
      .get_player_value(&self.pulled_index_param_name) {
        let destination = DeckPath::Hand(puller_index);
        let source = DeckPath::Hand(victim_index);
        let number_of_cards = game.deck(source).num_top_cards();
        if number_of_cards == 0 {
          return Ok(TaskProgressResult::TaskComplete);
        }
        let card_index = context.rng.gen_range(0..number_of_cards);
        let stack = game.deck_mut(source).take_at_index(card_index);
        game.deck_mut(destination).add(stack);
        Ok(TaskProgressResult::TaskComplete)
      }
      else {
        Ok(TaskProgressResult::NothingDone)
      }
	}

	fn label(&self) -> String {
		format!(
			"Clearing a player's task parameter state.",
		)
	}
}
