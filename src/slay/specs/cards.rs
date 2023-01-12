use enum_iterator::Sequence;

use crate::slay::specification::HeroType;
use crate::slay::specs::items::AnotherItemType;
use crate::slay::specs::magic::MagicSpell;
use crate::slay::specs::modifier::ModifierKinds;
use crate::slay::specs::monster::Monster;

use crate::slay::specs::hero::HeroAbilityType;

#[derive(Debug, Clone, Sequence, PartialEq, Copy)]
pub enum SlayCardSpec {
	HeroCard(HeroAbilityType),
	PartyLeader(HeroType),
	MonsterCard(Monster),
	MagicCard(MagicSpell),
	ModifierCard(ModifierKinds),
	Item(AnotherItemType),
	Challenge,
}

impl SlayCardSpec {
	pub fn label(&self) -> String {
		String::from("")
	}
	pub fn description(&self) -> String {
		String::from("")
	}
	pub fn image_path(&self) -> String {
		String::from("")
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
