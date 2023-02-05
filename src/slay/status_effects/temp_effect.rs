use crate::slay::state::turn::Turn;

use super::effect::PlayerStatusEffect;
use super::effect_entry::{EffectOrigin, PlayerStatusEffectEntry};

#[derive(Debug, Clone, Copy)]
pub enum EffectDuration {
	ForThisTurn(u32),
	UntilNextTurn(u32, usize),
	// Forever,
}

impl EffectDuration {
	pub fn until_next_turn(turn: Turn) -> EffectDuration {
		turn.until_next_turn()
	}
	pub fn for_this_turn(turn: Turn) -> EffectDuration {
		turn.for_this_turn()
	}

	// fn still_active(&self, turn: &state::Turn) -> bool {
	//     match self {
	//         ModifierDuration::UntilTurn(t, p) => turn. > *t || active_player > *p,
	//         ModifierDuration::Forever => true,
	//     }
	// }
}

#[derive(Debug, Clone, Copy)]
pub struct TemporaryPlayerStatusEffect {
	pub duration: EffectDuration,
	pub status_effect: PlayerStatusEffectEntry,
}

impl TemporaryPlayerStatusEffect {
	pub fn new(duration: EffectDuration, modifier: PlayerStatusEffect, origin: EffectOrigin) -> Self {
		Self {
			duration,
			status_effect: PlayerStatusEffectEntry { modifier, origin },
		}
	}
}
