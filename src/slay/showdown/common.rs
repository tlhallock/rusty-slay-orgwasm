use crate::slay::ids;
use crate::slay::modifiers::ModifierOrigin;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::specs::modifier::ModifierKinds;

use rand::Rng;

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

// TODO: rename to Roll....
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModificationPath {
	Roll,
	Challenger,
	Initiator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModificationOrigin {
	FromPlayer(ids::PlayerIndex, ModifierKinds),
	FromBuff(ModifierOrigin),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RollModification {
	pub origin: ModificationOrigin,
	pub amount: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RollModificationChoiceType {
	AddToRoll(ModifierKinds, i32, ModificationPath),
	RemoveFromRoll(ModifierKinds, i32, ModificationPath),
}

impl RollModificationChoiceType {
	pub fn get_path(&self) -> ModificationPath {
		match self {
			RollModificationChoiceType::AddToRoll(_, _, path) => *path,
			RollModificationChoiceType::RemoveFromRoll(_, _, path) => *path,
		}
	}
	pub fn from_card(spec: &ModifierKinds, amount: i32, path: ModificationPath) -> Self {
		if amount < 0 {
			RollModificationChoiceType::RemoveFromRoll(*spec, amount, path)
		} else {
			RollModificationChoiceType::AddToRoll(*spec, amount, path)
		}
	}
}

// It is a little awkward to have both this and the choice perspective...
#[derive(Debug, PartialEq, Clone)]
pub struct RollModificationChoice {
	pub choice_id: ids::ChoiceId,
	pub choice_type: RollModificationChoiceType,
}
