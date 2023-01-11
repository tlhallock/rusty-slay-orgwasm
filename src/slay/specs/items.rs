use enum_iterator::Sequence;

use crate::slay::specification::HeroType;

#[derive(Debug, Clone, Sequence)]
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

#[derive(Debug, Clone, Sequence)]
pub enum AnotherItemType {
	MaskCard(HeroType),
	NotMask(Item),
}
impl AnotherItemType {}
