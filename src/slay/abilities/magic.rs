use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specification::MagicSpell;
use crate::slay::state::game::Game;
use crate::slay::tasks::{PlayerTask, TaskProgressResult};

#[derive(Clone, Debug)]
pub struct MagicTask {
	spell: MagicSpell,
}

impl MagicTask {
	pub fn new(spell: MagicSpell) -> Self {
		Self { spell }
	}
}

impl PlayerTask for MagicTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, _game: &mut Game, _player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		match self.spell {}
		todo!()
	}

	fn label(&self) -> String {
		todo!()
	}
}
