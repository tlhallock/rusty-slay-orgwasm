use crate::slay::specification::HeroType;
use crate::slay::specs::magic::MagicSpell;

use crate::slay::showdown::roll_modification::{ModificationOrigin, RollModification};
use crate::slay::showdown::roll_state::RollReason;

use super::effect::{HeroStatusEffect, PlayerStatusEffect};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectOrigin {
	FromMagicCard(MagicSpell),
	FromHeroAbility,
	FromSlainMonster,
	// TODO: Could this be the hero type?
	FromPartyLeader(HeroType),
	FromItem,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerStatusEffectEntry {
	// TODO: rename to status effect
	pub modifier: PlayerStatusEffect,
	pub origin: EffectOrigin,
}

#[derive(Debug, Clone, Copy)]
pub struct HeroStatusEffectEntry {
	// TODO: rename to status effect
	pub effect: HeroStatusEffect,
	pub origin: EffectOrigin,
}

impl HeroStatusEffectEntry {
	pub fn new(effect: HeroStatusEffect, origin: EffectOrigin) -> Self {
		Self { effect, origin }
	}

	pub fn create_roll_modification(&self, reason: RollReason) -> Option<RollModification> {
		match self.effect {
			HeroStatusEffect::AddToRollForAbility(amount) => match reason {
				RollReason::UseHeroAbility(_) => Some(RollModification {
					origin: ModificationOrigin::FromBuff(EffectOrigin::FromItem),
					amount,
				}),
				_ => None,
			},
			_ => None,
		}
	}
}

impl PlayerStatusEffectEntry {
	pub fn new(modifier: PlayerStatusEffect, origin: EffectOrigin) -> Self {
		Self { modifier, origin }
	}

	pub fn create_roll_modification(&self, reason: RollReason) -> Option<RollModification> {
		match self.modifier {
			PlayerStatusEffect::AddToAllRolls(amount) => Some(RollModification {
				origin: ModificationOrigin::FromBuff(self.origin),
				amount,
			}),
			PlayerStatusEffect::AddToRollForAnyAbility(amount) => match reason {
				RollReason::UseHeroAbility(_) => Some(RollModification {
					origin: ModificationOrigin::FromBuff(self.origin),
					amount,
				}),
				_ => None,
			},
			PlayerStatusEffect::AddToRollForChallenge(amount) => match reason {
				// TODO: both?
				RollReason::Challenged | RollReason::Challenging => Some(RollModification {
					origin: ModificationOrigin::FromBuff(self.origin),
					amount,
				}),
				_ => None,
			},
			_ => None,
		}
	}
}
