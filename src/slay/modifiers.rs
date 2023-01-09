use crate::slay::state::game::Turn;

use super::{
	ids,
	specification::{HeroType, MagicSpell},
};

#[derive(Debug, Clone, Copy)]
pub enum ModifierDuration {
	ForThisTurn(u32),
	UntilNextTurn(u32, usize),
	Forever,
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

#[derive(Debug, Clone, Copy)]
pub enum ModifierOrigin {
	FromMagicCard(MagicSpell),
	FromHeroAbility(ids::CardId),
	FromSlainMonster,
	FromPartyLeader(ids::CardId),
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
	buffs: Vec<PlayerBuff>,
}
impl PlayerBuffs {
	pub(crate) fn clear_expired_modifiers(&mut self, turn: &Turn) {
		self.buffs.retain(|b| turn.still_active(&b.duration))
	}
	pub fn add_buff(
		&mut self, duration: ModifierDuration, modifier: PlayerModifier, origin: ModifierOrigin,
	) {
		self.buffs.push(PlayerBuff {
			duration,
			modifier,
			origin,
		});
	}
	pub fn add(&mut self, buff: PlayerBuff) {
		self.buffs.push(buff);
	}
	// pub fn add(&mut self, duration: ModifierDuration, modifier: PlayerModifier) {
	// 	self.buffs.push(PlayerBuff { duration, modifier });
	// }
	pub fn add_forever(&mut self, modifier: PlayerModifier, origin: ModifierOrigin) {
		self.add(PlayerBuff {
			duration: ModifierDuration::Forever,
			modifier,
			origin,
		});
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerModifier {
	UndestroyableHeros,
	PlayMagicOnDraw,
	ExtraActionPoint,
	AddToAllRolls(i32),
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
