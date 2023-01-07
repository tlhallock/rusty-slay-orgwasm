use crate::slay::{
	ids,
	specification::CardSpec,
	state::{game::Game, stack::CardSpecPerspective},
};
use rand::Rng;

use super::completion::RollCompletion;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModificationPath {
	Roll,
	Challenger,
	Initiator,
}

#[derive(Debug, Clone)]
pub struct RollModification {
	pub modifying_player_index: ids::PlayerIndex,
	pub card_id: ids::CardId,
	pub modification_amount: i32,
}

impl RollModification {
	pub fn to_perspective(&self, game: &Game) -> ModificationPerspective {
		let modifying_card = game.card(self.card_id).unwrap();
		let modifying_card_spec = CardSpecPerspective::new(&modifying_card.spec);
		ModificationPerspective {
			modifyer_name: game.players[self.modifying_player_index].name.to_owned(),
			modifying_card_spec,
			modification_amount: self.modification_amount,
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum RollModificationChoiceType {
	AddToRoll(CardSpecPerspective, i32),
	RemoveFromRoll(CardSpecPerspective, i32),
	Nothing(RollCompletion),
}

impl RollModificationChoiceType {
	pub fn from_card(spec: &CardSpec, amount: i32) -> Self {
		if amount < 0 {
			RollModificationChoiceType::RemoveFromRoll(CardSpecPerspective::new(&spec), amount)
		} else {
			RollModificationChoiceType::AddToRoll(CardSpecPerspective::new(&spec), amount)
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
	pub modifyer_name: String,
	pub modifying_card_spec: CardSpecPerspective,
	pub modification_amount: i32,
}
