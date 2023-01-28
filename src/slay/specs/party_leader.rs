use crate::slay::modifiers::PlayerModifier;
use crate::slay::specification::HeroType;

use crate::slay::specs::cards::card_type::SlayCardSpec;

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

				HeroType::Beserker => todo!(),
				HeroType::Necromancer => todo!(),
				HeroType::Druid => todo!(),
				HeroType::Warrior => todo!(),
				HeroType::Sorcerer => todo!(),
			},
			_ => None,
		}
	}
}
