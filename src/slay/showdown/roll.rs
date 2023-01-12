use crate::slay::ids;
use crate::slay::modifiers::ModifierOrigin;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::specs::modifier::ModifierKinds;

use rand::Rng;

use super::roll_modification::RollModification;

// Only the party needs stacks...

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roll {
	pub die1: u32,
	pub die2: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ChallengeReason {
	PlaceHeroCard(SlayCardSpec),
	PlaceItem(SlayCardSpec),
	CastMagic(SlayCardSpec),
}

impl Roll {
	pub fn create_from(rng: &mut rand::rngs::ThreadRng) -> Self {
		Roll {
			die1: rng.gen_range(1..=6),
			die2: rng.gen_range(1..=6),
		}
	}

	pub fn calculate_total(&self, modifications: &[RollModification]) -> i32 {
		(self.die1 as i32) + (self.die2 as i32) + modifications.iter().map(|m| m.amount).sum::<i32>()
	}
}
