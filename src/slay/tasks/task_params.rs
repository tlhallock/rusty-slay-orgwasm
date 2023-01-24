use std::collections::HashMap;
use std::io::BufWriter;
use std::io::Write;

use crate::slay::ids;
use crate::slay::state::summarizable::Summarizable;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TaskParamName {
	PanChuckFirstCard,
	PanChuckSecondCard,
	LuckBuckyVictim,
	LuckBuckyCard,
	PlayerToStealFrom,
	CardToSteal,
	PlayerToPullFrom,
	PlayerToGiveItem,
	StackToPlaceItemOn,
	MeowzioVictim,
	MeowzioCard,
	PlayerToDestroy,
	CardToDestroy,
	SilentShadowVictim,
	SilentShadowCard,
	SlipperyPawsVictim,
	SlipperyPawsVictimPulledCard1,
	SlipperyPawsVictimPulledCard2,
	SlyPickinsVictim,
	SlyPickinsCard,
	ForcedExchangeVictim,
	ForcedExchangeVictimCard,
	ForcedExchangeSelf,
	ForcedExchangeVictimDonationCard,
	ShadowClawVictim,
	HeavyBearVictim,
	MellowDeeVictim,
	DodgyDealerVictim,
	HopperVictim,
}

#[derive(Debug, Default, Clone)]
pub struct TaskParams {
	// These should probably be card paths, right?
	pub players: HashMap<TaskParamName, ids::PlayerIndex>,
	// None of the player did not choose a card.
	pub cards: HashMap<TaskParamName, Option<ids::CardId>>,
	index: HashMap<TaskParamName, usize>,
}

impl TaskParams {
	pub fn clear(&mut self) {
		self.players.clear();
		self.cards.clear();
		self.index.clear();
	}
}

impl Summarizable for TaskParams {
	fn summarize<W: Write>(
		&self,
		f: &mut BufWriter<W>,
		indentation_level: u32,
	) -> Result<(), std::io::Error> {
		if !self.cards.is_empty() {
			for _ in 0..indentation_level {
				write!(f, "  ")?;
			}
			write!(f, "card params: ")?;
			for (name, value) in self.cards.iter() {
				write!(f, "{:?}->{:?}, ", name, value)?;
			}
			writeln!(f)?;
		}
		if !self.players.is_empty() {
			for _ in 0..indentation_level {
				write!(f, "  ")?;
			}
			write!(f, "player params: ")?;
			for (name, value) in self.players.iter() {
				write!(f, "{:?}->{:?}, ", name, value)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}
