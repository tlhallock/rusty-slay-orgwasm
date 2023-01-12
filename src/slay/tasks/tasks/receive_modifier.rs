use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::modifiers::ModifierOrigin;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Debug, Clone)]
pub struct ReceiveModifier {
	modifier: PlayerModifier,
	origin: ModifierOrigin,
}

impl ReceiveModifier {
	pub fn create(modifier: PlayerModifier, origin: ModifierOrigin) -> Box<dyn PlayerTask> {
		Box::new(Self { modifier, origin })
	}
}

impl PlayerTask for ReceiveModifier {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		_game: &mut Game,
		_player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		// game.players[player_index]
		// 	.temporary_buffs
		// 	.add_forever(self.modifier.to_owned(), self.origin);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Player is receiving modifier {:?}", self.modifier)
	}
}
