use crate::slay::specification::HeroType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatusEffect {
	PlayMagicOnDraw,
	PlayItemOnDraw,

	ItemsCannotBeChallenged,
	NoCardsCanBeChallenged,
	NoCardsCanBeStolen,
	NoCardsCanBeDestroyed,

	UndestroyableHeros,
	ExtraActionPoint,
	DrawOnSuccessfulAbility,
	DiscardOnChallenge,
	DrawOnDestroy,
	StealInsteadOfSacrifice,
	RevealModifiersAndDrawAgain,
	DrawOnPlayMagic,
	ModifierBonus,

	DrawOnModify,
	AddOnModify,

	AddToRollForAttack(i32),
	AddToAllRolls(i32),
	AddToRollForAnyAbility(i32),
	AddToRollForChallenge(i32),
}

// Rename this to card modifier, or hero modifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeroStatusEffect {
	Mask(HeroType),
	AddToRollForAbility(i32),
	DrawOnUnsuccessfulRollForAbility(u32),
	DiscardOnSuccessfulRollForAbility(u32),
	RemoveAbility,
	SacrificeMeInstead,
}
