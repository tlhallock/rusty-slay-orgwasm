use crate::slay::ids;
use crate::slay::specification::CardSpec;
use crate::slay::state::game::Game;
use crate::slay::state::stack::CardSpecPerspective;

use rand::Rng;

// Only the party needs stacks...

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roll {
	pub die1: u32,
	pub die2: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ChallengeReason {
	PlaceHeroCard(CardSpecPerspective),
	PlaceItem(CardSpecPerspective),
	CastMagic(CardSpecPerspective),
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
pub struct RollModification {
	pub modifying_player_index: ids::PlayerIndex,
	pub card_id: ids::CardId,
	// This cannot be a card path: because it starts in the hand and ends in the discard pile...
	// pub card_path: CardPath,
	pub modification_amount: i32,
}

impl RollModification {
	pub fn to_perspective(&self, game: &Game) -> ModificationPerspective {
		let modifying_card = game.find_card(self.card_id).unwrap();
		ModificationPerspective {
			modifier_name: game.get_player_name(self.modifying_player_index),
			modifying_card_spec: CardSpecPerspective::new(&modifying_card.get_spec()),
			modification_amount: self.modification_amount,
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum RollModificationChoiceType {
	AddToRoll(CardSpecPerspective, i32, ModificationPath),
	RemoveFromRoll(CardSpecPerspective, i32, ModificationPath),
}

impl RollModificationChoiceType {
	pub fn get_path(&self) -> ModificationPath {
		match self {
			RollModificationChoiceType::AddToRoll(_, _, path) => *path,
			RollModificationChoiceType::RemoveFromRoll(_, _, path) => *path,
		}
	}
	pub fn from_card(spec: &CardSpec, amount: i32, path: ModificationPath) -> Self {
		if amount < 0 {
			RollModificationChoiceType::RemoveFromRoll(CardSpecPerspective::new(spec), amount, path)
		} else {
			RollModificationChoiceType::AddToRoll(CardSpecPerspective::new(spec), amount, path)
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
	pub modifier_name: String,
	pub modifying_card_spec: CardSpecPerspective,
	pub modification_amount: i32,
}
