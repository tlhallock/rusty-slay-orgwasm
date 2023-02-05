use crate::slay::{
	specification::HeroType,
	status_effects::{
		effect::PlayerStatusEffect,
		effect_entry::{EffectOrigin, PlayerStatusEffectEntry},
	},
};

impl HeroType {
	pub fn get_leader_effect(&self) -> Option<PlayerStatusEffect> {
		match self {
			HeroType::Bard => Some(PlayerStatusEffect::AddToRollForAnyAbility(1)),
			HeroType::Wizard => Some(PlayerStatusEffect::DrawOnPlayMagic),
			HeroType::Fighter => Some(PlayerStatusEffect::AddToRollForChallenge(2)),
			HeroType::Gaurdian => Some(PlayerStatusEffect::ModifierBonus),
			HeroType::Ranger => Some(PlayerStatusEffect::AddToRollForAttack(1)),
			HeroType::Thief => None,

			HeroType::Beserker => todo!(),
			HeroType::Necromancer => todo!(),
			HeroType::Druid => todo!(),
			HeroType::Warrior => todo!(),
			HeroType::Sorcerer => todo!(),
		}
	}
	pub fn get_leader_effect_entry(&self) -> Option<PlayerStatusEffectEntry> {
		self
			.get_leader_effect()
			.map(|effect| PlayerStatusEffectEntry {
				modifier: effect,
				// TODO: Why an id?
				origin: EffectOrigin::FromPartyLeader(*self),
			})
	}

	// pub fn create_party_leader_buffs(&self) -> Option<PlayerStatusEffect> {
	// 	match self {
	// 		SlayCardSpec::PartyLeader(hero_type) => ,
	// 		_ => None,
	// 	}
	// }
}
