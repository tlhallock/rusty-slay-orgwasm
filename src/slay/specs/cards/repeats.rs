use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::specs::items::{AnotherItemType, Item};
use crate::slay::specs::magic::MagicSpell;
use crate::slay::specs::modifier::ModifierKinds;

impl SlayCardSpec {
	pub fn repeat(&self) -> u32 {
		match self {
			SlayCardSpec::HeroCard(_) | SlayCardSpec::PartyLeader(_) | SlayCardSpec::MonsterCard(_) => 1,
			SlayCardSpec::MagicCard(spell) => match spell {
				MagicSpell::EnganglingTrap
				| MagicSpell::CriticalBoost
				| MagicSpell::DestructiveSpell
				| MagicSpell::WindsOfChange
				| MagicSpell::EnchangedSpell => 2,
				MagicSpell::ForcedExchange | MagicSpell::ForcefulWinds | MagicSpell::CallToTheFallen => 1,
			},
			SlayCardSpec::ModifierCard(modifier_kind) => match modifier_kind {
				ModifierKinds::Plus4 => 4,
				ModifierKinds::Plus3Minus1 => 4,
				ModifierKinds::Plus2Minus2 => 9,
				ModifierKinds::Plus1Minus3 => 4,
				ModifierKinds::Minus4 => 4,
			},
			SlayCardSpec::Item(item_card) => match item_card {
				AnotherItemType::MaskCard(_) => 1,
				AnotherItemType::NotMask(item) => match item {
					Item::DecoyDoll => 1,
					Item::ReallyBigRing => 2,
					Item::ParticularlyRustyCoin => 2,
					Item::SealingKey => 1,
					Item::SuspiciouslyShinyCoin => 1,
					Item::CurseOfTheSnakesEyes => 2,
				},
			},
			SlayCardSpec::Challenge => 8,
		}
	}
}
