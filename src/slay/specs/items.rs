use enum_iterator::Sequence;

use crate::slay::specification::HeroType;

use super::cards::SlayCardSpec;

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
	pub fn item_type(&self) {
		//
	}
}

#[derive(Debug, Clone, Sequence, PartialEq, Copy)]
pub enum AnotherItemType {
	MaskCard(HeroType),
	NotMask(Item),
}
impl AnotherItemType {
	pub fn label(&self) -> String {
		SlayCardSpec::Item(*self).label()
	}
}
