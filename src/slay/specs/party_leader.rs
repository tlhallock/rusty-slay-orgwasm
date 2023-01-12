use crate::slay::modifiers::PlayerModifier;
use crate::slay::specification::HeroType;

use super::cards::SlayCardSpec;

impl SlayCardSpec {
	pub fn create_party_leader_buffs(&self) -> Option<PlayerModifier> {
		match self {
			SlayCardSpec::PartyLeader(hero_type) => match hero_type {
				HeroType::Bard => Some(PlayerModifier::AddToRollForAnyAbility(1)),
				HeroType::Wizard => Some(PlayerModifier::DrawOnPlayMagic),
				HeroType::Fighter => Some(PlayerModifier::AddToRollForChallenge(2)),
				HeroType::Gaurdian => Some(PlayerModifier::ModifierBonus),
				HeroType::Ranger => Some(PlayerModifier::AddToRollForAttack(1)),
				HeroType::Thief => None,
			},
			_ => None,
		}
	}
}
