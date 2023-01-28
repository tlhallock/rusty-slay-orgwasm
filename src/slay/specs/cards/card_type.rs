use enum_iterator::Sequence;

use crate::slay::specification::HeroType;
use crate::slay::specs::items::AnotherItemType;
use crate::slay::specs::magic::MagicSpell;
use crate::slay::specs::modifier::ModifierKinds;
use crate::slay::specs::monster::Monster;

use crate::slay::specs::hero::HeroAbilityType;

#[derive(Debug, Clone, Sequence, PartialEq, Copy)]
pub enum ChallengeType {
	Standard,
	Warrior,
}

// Rename this: remove the Slay
#[derive(Debug, Clone, Sequence, PartialEq, Copy)]
pub enum SlayCardSpec {
	HeroCard(HeroAbilityType),
	PartyLeader(HeroType),
	MonsterCard(Monster),
	MagicCard(MagicSpell),
	ModifierCard(ModifierKinds),
	Item(AnotherItemType),
	Challenge, /*(ChallengeType)*/
}

impl SlayCardSpec {
	pub fn hero_type(&self) -> Option<HeroType> {
		match self {
			SlayCardSpec::HeroCard(hero_card) => Some(hero_card.hero_type()),
			SlayCardSpec::PartyLeader(hero_type) => Some(*hero_type),
			_ => None,
		}
	}
}

// // This should be done automatically...
// pub fn cards() -> [SlayCardSpec; 3] {
// 	[
// 	MonsterC
// 	MonsterCard(Monster),
// 	MagicCard(MagicSpell),
// 	ModifierCard(ModifierKinds),
// 	Item(AnotherItemType),
// 	Challenge,
// 	]
// }
