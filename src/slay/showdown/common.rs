use crate::slay::{ids, specification::CardSpec};
use rand::Rng;

// Only the party needs stacks...

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roll {
	pub die1: u32,
	pub die2: u32,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum ChallengeReason {
	PlaceHeroCard(CardSpec),
	PlaceItem(CardSpec),
	CastMagic(CardSpec),
}

impl Roll {
	pub fn create_from(rng: &mut rand::rngs::ThreadRng) -> Self {
		Roll {
			die1: rng.gen_range(1..=6),
			die2: rng.gen_range(1..=6),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModificationPath {
	Roll,
	Challenger,
	Initiator,
}

#[derive(Debug, Clone)]
pub struct RollModification {
	pub modifying_player_index: usize,
	pub card_id: ids::CardId,
	pub modification_amount: i32,
}
