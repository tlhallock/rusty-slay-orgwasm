use crate::slay::ids;
use crate::slay::modifiers::ModifierDuration;
use crate::slay::specification;

use std::fmt::Debug;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Turn {
	turn_number: u32,
	round_number: u32,
	// lol, the one that has to be small is the large one...
	player_index: usize,
}

impl Turn {
	pub(crate) fn for_this_turn(&self) -> ModifierDuration {
		ModifierDuration::ForThisTurn(self.turn_number)
	}
	pub(crate) fn until_next_turn(&self) -> ModifierDuration {
		ModifierDuration::UntilNextTurn(self.round_number + 1, self.player_index)
	}

	pub(crate) fn set_active_player(&mut self, player_index: ids::PlayerIndex) {
		self.player_index = player_index;
	}

	pub fn still_active(&self, duration: &ModifierDuration) -> bool {
		match duration {
			ModifierDuration::ForThisTurn(turn_number) => *turn_number == self.turn_number,
			ModifierDuration::UntilNextTurn(round_number, player_index) => {
				self.round_number <= *round_number
					|| (self.round_number == round_number + 1 && self.player_index < *player_index)
			}
		}
	}

	pub fn increment(&mut self, number_of_players: usize) {
		self.player_index += 1;
		self.turn_number += 1;
		if self.player_index < number_of_players {
			log::info!("Incremented turn to {:?}", &self);
			return;
		}
		self.player_index = 0;
		self.round_number += 1;
		log::info!("Incremented round to {:?}", &self);
	}

	pub fn over_the_limit(&self) -> bool {
		self.round_number >= specification::MAX_TURNS
	}
	pub fn active_player_index(&self) -> ids::PlayerIndex {
		self.player_index as ids::PlayerIndex
	}
}
