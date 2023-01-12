







use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;












use crate::slay::specs::magic::MagicSpell;

use crate::slay::state::game::Game;


use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

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
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		self.spell.perform_spell(context, game, player_index)
	}

	fn label(&self) -> String {
		format!("Cast the {:?} spell", self.spell)
	}
}
