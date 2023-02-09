use crate::slay::specification::HeroType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatusEffect {
	// Orthus: Each time you DRAW a Magic card, you may play it immediately.
	PlayMagicOnDraw, // done
	// Malammoth: Each time you DRAW an Item card, you may play it immediately.
	PlayItemOnDraw, // done

	ItemsCannotBeChallenged, // done
	NoCardsCanBeChallenged,  // done
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeroStatusEffect {
	Mask(HeroType),
	AddToRollForAbility(i32),
	DrawOnUnsuccessfulRollForAbility(u32),
	DiscardOnSuccessfulRollForAbility(u32),
	RemoveAbility,
	SacrificeMeInstead,
}
