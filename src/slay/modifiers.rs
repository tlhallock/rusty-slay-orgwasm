use crate::slay::state::turn::Turn;

use crate::slay::ids;
use crate::slay::modifier_visitors::ModifierVisitor;
use crate::slay::specification::HeroType;
use crate::slay::specs::magic::MagicSpell;

#[derive(Debug, Clone, Copy)]
pub enum ModifierDuration {
	ForThisTurn(u32),
	UntilNextTurn(u32, usize),
	// Forever,
}
impl ModifierDuration {
	pub fn until_next_turn(turn: Turn) -> ModifierDuration {
		turn.until_next_turn()
	}
	pub fn for_this_turn(turn: Turn) -> ModifierDuration {
		turn.for_this_turn()
	}

	// fn still_active(&self, turn: &state::Turn) -> bool {
	//     match self {
	//         ModifierDuration::UntilTurn(t, p) => turn. > *t || active_player > *p,
	//         ModifierDuration::Forever => true,
	//     }
	// }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModifierOrigin {
	FromMagicCard(MagicSpell),
	FromHeroAbility,
	FromSlainMonster,
	FromPartyLeader(ids::CardId),
	FromItem,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerBuff {
	duration: ModifierDuration,
	modifier: PlayerModifier,
	origin: ModifierOrigin,
}

impl PlayerBuff {
	pub fn new(duration: ModifierDuration, modifier: PlayerModifier, origin: ModifierOrigin) -> Self {
		Self {
			duration,
			modifier,
			origin,
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct PlayerBuffs {
	temporary_buffs: Vec<PlayerBuff>,
}
impl PlayerBuffs {
	pub(crate) fn tour_buffs(&self, visitor: &mut dyn ModifierVisitor) {
		for buff in self.temporary_buffs.iter() {
			visitor.visit_player_modifier(buff.modifier, buff.origin)
		}
	}

	pub(crate) fn clear_expired_modifiers(&mut self, turn: &Turn) {
		self
			.temporary_buffs
			.retain(|b| turn.still_active(&b.duration))
	}
	pub fn add_buff(
		&mut self,
		duration: ModifierDuration,
		modifier: PlayerModifier,
		origin: ModifierOrigin,
	) {
		self.temporary_buffs.push(PlayerBuff {
			duration,
			modifier,
			origin,
		});
	}
	pub fn add(&mut self, buff: PlayerBuff) {
		self.temporary_buffs.push(buff);
	}
	// pub fn add(&mut self, duration: ModifierDuration, modifier: PlayerModifier) {
	// 	self.buffs.push(PlayerBuff { duration, modifier });
	// }
	// pub fn add_forever(&mut self, modifier: PlayerModifier, origin: ModifierOrigin) {
	// 	self.add(PlayerBuff {
	// 		duration: ModifierDuration::Forever,
	// 		modifier,
	// 		origin,
	// 	});
	// }

	// pub(crate) fn collect_roll_buffs(&self, ret: &mut Vec<RollModification>) {
	// 	for modifier in self.temporary_buffs.iter() {
	// 		if let PlayerModifier::AddToAllRolls(amount) = &modifier.modifier {
	// 			ret.push(RollModification {
	// 				modification_origin: ModificationOrigin::FromBuff(modifier.origin),
	// 				modification_amount: *amount,
	// 			})
	// 		}
	// 	}
	// }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerModifier {
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
pub enum ItemModifier {
	Mask(HeroType),
	AddToRollForAbility(i32),
	DrawOnUnsuccessfulRollForAbility(u32),
	DiscardOnSuccessfulRollForAbility(u32),
	RemoveAbility,
	SacrificeMeInstead,
}
