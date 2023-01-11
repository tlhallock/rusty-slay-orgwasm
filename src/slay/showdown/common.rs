use crate::slay::ids;
use crate::slay::modifiers::ModifierOrigin;
use crate::slay::specification::CardSpec;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::specs::modifier::ModifierKinds;
use crate::slay::state::game::Game;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModificationPath {
	Roll,
	Challenger,
	Initiator,
}

#[derive(Debug, Clone)]
pub enum ModificationOrigin {
	FromPlayer(ids::PlayerIndex, ModifierKinds),
	FromBuff(ModifierOrigin),
}

#[derive(Debug, Clone)]
pub struct RollModification {
	pub modification_origin: ModificationOrigin,
	pub modification_amount: i32,
}

impl RollModification {
	pub fn to_perspective(&self, game: &Game) -> ModificationPerspective {
		// TODO: reference count the game and pass it all the way down the ui tree.
		// Then we can fill in information like the player names as we go...
		// Perspective =/= ui
		// Static Perspective Information...
		// Dynamic Perspective information...

		// let modifying_card = game.find_card(self.card_id).unwrap();
		ModificationPerspective {
			modifier_name: None,       // game.get_player_name(self.modifying_player_index),
			modifying_card_spec: None, // modifying_card.card_type,
			modification_amount: self.modification_amount,
		}
	}
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

#[derive(Debug, PartialEq, Clone)]
pub struct ModificationPerspective {
	// TODO: refactoring this
	pub modifier_name: Option<String>,
	pub modifying_card_spec: Option<SlayCardSpec>,
	pub modification_amount: i32,
}
