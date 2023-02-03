use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufWriter;
use std::io::Write;

use crate::slay::ids;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::state::summarizable::Summarizable;

/*
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TaskPlayerParameterName {

}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TaskCardParameterName {

}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TaskSetParameterName {

}*/

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TaskParamName {
	TipsyTootieVictim,
	TipsyTootieCard,
	ButtonsVictim,
	ButtonsCard,
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
	MellowDeeCard,
	DodgyDealerVictim,
	HopperVictim,
	WigglesVictim,
	WigglesCard,
	SnowballCard,
	BearClawVictim,
	BearClawCard,
	FuryKnuckleVictim,
	FuryKnuckleCard,
	QuickDrawCard1,
	QuickDrawCard2,
	ShurikittyVictim,
	ShurikittyCard,
	SharpFoxVictim,
}

impl TaskParamName {
	pub fn prompt(&self) -> &'static str {
		match self {
			TaskParamName::ButtonsVictim => {
				"Who would you like to pull from? (If it is magic, you can play it immediately.)"
			}
			TaskParamName::ButtonsCard => "N/A",
			TaskParamName::PanChuckFirstCard => "N/A",
			TaskParamName::PanChuckSecondCard => "N/A",
			TaskParamName::LuckBuckyVictim => {
				"Who would you like to pull from? (If it is a hero card, you can play it immediately.)"
			}
			TaskParamName::LuckBuckyCard => "N/A",
			TaskParamName::PlayerToStealFrom => "Which player would you like to steal from?",
			TaskParamName::CardToSteal => "Which hero card would you like to steal?",
			TaskParamName::PlayerToPullFrom => "Choose a player to pull from.",
			TaskParamName::PlayerToGiveItem => {
				"Which player has the hero card you would like to place this item on?"
			}
			TaskParamName::StackToPlaceItemOn => "Which hero card would you like to place this item on?",
			TaskParamName::MeowzioVictim => "Choose a player to steal and pull from.",
			TaskParamName::MeowzioCard => "Which hero card would you like to steal?",
			TaskParamName::PlayerToDestroy => "Which player should destroy a hero card?",
			TaskParamName::CardToDestroy => "Which hero card would you like to destroy?",
			TaskParamName::SilentShadowVictim => "Who's hand do you want to see?",
			TaskParamName::SilentShadowCard => "Which hero card would you like to take?",
			TaskParamName::SlipperyPawsVictim => {
				"Choose a player to pull 2 cards from, you will have to discard one of them."
			}
			TaskParamName::SlipperyPawsVictimPulledCard1 => "N/A",
			TaskParamName::SlipperyPawsVictimPulledCard2 => "N/A",
			TaskParamName::SlyPickinsVictim => {
				"Who do you want to steal from? (If it is magic, you can play it immediately.)"
			}
			TaskParamName::SlyPickinsCard => "N/A",
			TaskParamName::ShadowClawVictim => "Choose a player to steal from.",
			TaskParamName::HeavyBearVictim => "Who should discard 2 cards?",
			TaskParamName::MellowDeeCard => "N/A",
			TaskParamName::DodgyDealerVictim => "Who do you want to trade hands with?",
			TaskParamName::HopperVictim => "Who should sacrifice a hero card?",
			TaskParamName::WigglesVictim => "Wiggles: Who do you want to steal a hero card from?",
			TaskParamName::WigglesCard => "Which hero card would you like to steal?",
			TaskParamName::SnowballCard => "N/A",
			TaskParamName::BearClawVictim => {
				"Who would you like to pull from? (If you pull a hero, you can play it immediately.)"
			}
			TaskParamName::BearClawCard => "N/A",
			TaskParamName::FuryKnuckleVictim => {
				"Who would you like to pull from? (If it is a challenge card, you can pull again.)"
			}
			TaskParamName::FuryKnuckleCard => "N/A",
			TaskParamName::QuickDrawCard1 => "N/A",
			TaskParamName::QuickDrawCard2 => "N/A",
			TaskParamName::ShurikittyVictim => {
				"Who would you like to destroy a card from? (You will receive that card's items.)"
			}
			TaskParamName::ShurikittyCard => {
				"Which hero card would you like to destroy? (You will receive that card's items.)"
			}
			TaskParamName::ForcedExchangeVictim => "Choose a player to forcefully exchange heros with.",
			TaskParamName::ForcedExchangeVictimCard => "Which hero card would you like to steal?",
			TaskParamName::ForcedExchangeSelf => "N/A",
			TaskParamName::ForcedExchangeVictimDonationCard => {
				"Which hero card would you like to move to their hand?"
			}
			TaskParamName::TipsyTootieVictim => "Who would you like to steal a card from?",
			TaskParamName::TipsyTootieCard => "Which card would you like to steal?",
			TaskParamName::SharpFoxVictim => "Whose hand would you like to see?",
		}
	}
}

#[derive(Debug, Default, Clone)]
pub struct TaskParams {
	// These should probably be card paths, right?
	pub players: HashMap<TaskParamName, ids::PlayerIndex>,
	// None of the player did not choose a card.
	pub cards: HashMap<TaskParamName, Option<ids::CardId>>,
	pub sets: HashMap<TaskParamName, HashSet<SlayCardSpec>>,
}

impl TaskParams {
	pub fn clear(&mut self) {
		self.players.clear();
		self.cards.clear();
		self.sets.clear();
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
		if !self.sets.is_empty() {
			for _ in 0..indentation_level {
				write!(f, "  ")?;
			}
			write!(f, "sets params: ")?;
			for (name, value) in self.sets.iter() {
				write!(f, "{:?}->{:?}, ", name, value)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}
