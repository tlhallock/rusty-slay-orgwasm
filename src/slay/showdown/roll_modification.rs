use crate::slay::ids;
use crate::slay::specs::modifier::ModifierKinds;
use crate::slay::status_effects::effect_entry::EffectOrigin;

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
	FromBuff(EffectOrigin),
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

#[derive(Debug, PartialEq, Clone)]
pub struct RollModificationChoice {
	pub choice_id: ids::ChoiceId,
	pub choice_type: RollModificationChoiceType,
}
