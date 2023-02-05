use enum_iterator::Sequence;

use crate::slay::specification::{HeroType, ItemType};

use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::status_effects::effect::HeroStatusEffect;
use crate::slay::status_effects::effect_entry::{EffectOrigin, HeroStatusEffectEntry};

#[derive(Debug, Clone, Sequence, PartialEq, Copy)]
pub enum Item {
	DecoyDoll,
	ReallyBigRing,
	ParticularlyRustyCoin,
	SealingKey,
	SuspiciouslyShinyCoin,
	CurseOfTheSnakesEyes,
}

impl Item {
	pub fn item_type(&self) -> ItemType {
		match self {
			Item::DecoyDoll | Item::ReallyBigRing | Item::ParticularlyRustyCoin => ItemType::Blessed,
			Item::SealingKey | Item::SuspiciouslyShinyCoin | Item::CurseOfTheSnakesEyes => {
				ItemType::Cursed
			}
		}
	}
}

#[derive(Debug, Clone, Sequence, PartialEq, Copy)]
pub enum AnotherItemType {
	MaskCard(HeroType),
	NotMask(Item),
}

impl AnotherItemType {
	pub fn label(&self) -> &'static str {
		SlayCardSpec::Item(*self).label()
	}
	pub fn hero_effect(&self) -> HeroStatusEffect {
		match self {
			AnotherItemType::MaskCard(hero_type) => HeroStatusEffect::Mask(*hero_type),
			AnotherItemType::NotMask(item) => match item {
				Item::DecoyDoll => HeroStatusEffect::SacrificeMeInstead,
				Item::ReallyBigRing => HeroStatusEffect::AddToRollForAbility(2),
				Item::ParticularlyRustyCoin => HeroStatusEffect::DrawOnUnsuccessfulRollForAbility(1),
				Item::SealingKey => HeroStatusEffect::RemoveAbility,
				Item::SuspiciouslyShinyCoin => HeroStatusEffect::DiscardOnSuccessfulRollForAbility(1),
				Item::CurseOfTheSnakesEyes => HeroStatusEffect::AddToRollForAbility(-2),
			},
		}
	}
	pub fn hero_effect_entry(&self) -> HeroStatusEffectEntry {
		HeroStatusEffectEntry {
			effect: self.hero_effect(),
			origin: EffectOrigin::FromItem,
		}
	}
}
