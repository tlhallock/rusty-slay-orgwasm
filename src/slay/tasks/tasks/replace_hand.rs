use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Debug, Clone)]
pub struct ReplaceHandTask {}

impl PlayerTask for ReplaceHandTask {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game
			.discard
			.extend(game.players[player_index].hand.drain(..));
		game.replentish_for(5);
		game.players[player_index]
			.hand
			.extend(game.draw.drain(0..5));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Replace your hand with 5 new cards.".to_string()
	}
}
