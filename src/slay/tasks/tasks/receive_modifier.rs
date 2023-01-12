use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::modifiers::ModifierDuration;
use crate::slay::modifiers::ModifierOrigin;
use crate::slay::modifiers::PlayerBuff;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::state::game::Game;
use crate::slay::state::turn::Turn;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Debug, Clone)]
enum DurationSpec {
	ForThisTurn,
	UntilNextTurn,
}

impl DurationSpec {
	fn create_duration(&self, turn: &Turn) -> ModifierDuration {
		match self {
			DurationSpec::ForThisTurn => turn.for_this_turn(),
			DurationSpec::UntilNextTurn => turn.until_next_turn(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct ReceiveModifier {
	duration: DurationSpec,
	modifier: PlayerModifier,
	origin: ModifierOrigin,
}

impl ReceiveModifier {
	pub fn for_this_turn(modifier: PlayerModifier, origin: ModifierOrigin) -> Box<dyn PlayerTask> {
		Box::new(Self {
			modifier,
			origin,
			duration: DurationSpec::ForThisTurn,
		})
	}
	pub fn until_next_turn(modifier: PlayerModifier, origin: ModifierOrigin) -> Box<dyn PlayerTask> {
		Box::new(Self {
			modifier,
			origin,
			duration: DurationSpec::UntilNextTurn,
		})
	}
}

impl PlayerTask for ReceiveModifier {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let duration = self.duration.create_duration(game.get_turn());
		game.players[player_index]
			.temporary_buffs
			.add(PlayerBuff::new(
				duration,
				self.modifier.to_owned(),
				self.origin.to_owned(),
			));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Player is receiving modifier {:?}", self.modifier)
	}
}
