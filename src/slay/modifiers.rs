
use crate::slay::state::game::Turn;

#[derive(Debug, Clone, Copy)]
pub enum ModifierDuration {
	UntilTurn(u32, u32),
	Forever,
}
impl ModifierDuration {
	pub fn until_next_turn(turn_number: u32, active_player: u32) -> ModifierDuration {
		ModifierDuration::UntilTurn(turn_number + 1, active_player)
	}
	pub fn for_this_turn(turn_number: u32, active_player: u32) -> ModifierDuration {
		ModifierDuration::UntilTurn(turn_number, active_player + 1)
	}

	// fn still_active(&self, turn: &state::Turn) -> bool {
	//     match self {
	//         ModifierDuration::UntilTurn(t, p) => turn. > *t || active_player > *p,
	//         ModifierDuration::Forever => true,
	//     }
	// }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerBuff {
	duration: ModifierDuration,
	modifier: PlayerModifier,
}

impl PlayerBuff {
	pub fn new(duration: ModifierDuration, modifier: PlayerModifier) -> Self {
		Self { duration, modifier }
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
	pub fn add(&mut self, duration: ModifierDuration, modifier: PlayerModifier) {
		self.buffs.push(PlayerBuff { duration, modifier });
	}
	pub fn add_forever(&mut self, modifier: PlayerModifier) {
		self.add(ModifierDuration::Forever, modifier);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerModifier {
	UndestroyableHeros,
	PlayMagicOnDraw,
	ExtraActionPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemModifier {}
