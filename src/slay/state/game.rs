use crate::slay::choices::CardPath;
use crate::slay::choices::ChoicesPerspective;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::ids;
use crate::slay::showdown::challenge::ChallengePerspective;
use crate::slay::showdown::current_showdown::CurrentShowdown;
use crate::slay::showdown::offer::OfferChallengesPerspective;
use crate::slay::showdown::roll_state::RollPerspective;
use crate::slay::specs::visibility::Perspective;
use crate::slay::specs::visibility::VisibilitySpec;
use crate::slay::state::deck::Deck;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::deck::DeckPerspective;
use crate::slay::state::player::Player;
use crate::slay::state::player::PlayerPerspective;
use crate::slay::state::stack::Card;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::task_params::TaskParamName;

use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;
use std::iter::Iterator;

use super::deck::DeckSpec;
use super::turn::Turn;

#[derive(Clone, Debug)]
pub struct Game {
	pub players: Vec<Player>,
	pub showdown: CurrentShowdown,
	turn: Turn,
	// decks should reduce visibility and use deckpath...
	pub draw: Deck,
	pub discard: Deck,
	pub monsters: Deck,
	pub leaders: Deck,
	pub next_monsters: Deck,
}
/*
	Game <- domain state stored in db
	GameStaticInformation <- game specific information that only needs to be sent once
	GamePerspective <- game state with information not known to a given player hidden
												goes over the network
	GameDisplayable <- game layed out in a fashion to make displaying easy within a ui

*/

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerStaticInformation {
	pub name: String,
	pub leader: Card, // <-- This is not currently visible...
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameStaticInformation {
	pub players: Vec<PlayerStaticInformation>,
	pub player_index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GamePerspective {
	pub players: Vec<PlayerPerspective>,
	pub decks: Vec<DeckPerspective>,
	pub choices: Option<ChoicesPerspective>,
	pub turn: Turn,
	pub roll: Option<RollPerspective>,
	pub offer: Option<OfferChallengesPerspective>,
	pub challenge: Option<ChallengePerspective>,
}

impl GamePerspective {
	pub fn rotated_players(&self, statics: &GameStaticInformation) -> Vec<&PlayerPerspective> {
		let mut ret: Vec<&PlayerPerspective> = self.players.iter().collect();
		let position = ret.iter().position(|p| p.is_me(statics));
		if let Some(index) = position {
			ret.rotate_left(index)
		}
		ret
	}
}

impl Game {
	pub fn clear_expired_modifiers(&mut self) {
		self
			.players
			.iter_mut()
			.for_each(|player| player.clear_expired_modifiers(&self.turn))
	}
	pub fn active_player_index(&self) -> ids::PlayerIndex {
		self.turn.active_player_index()
	}
	pub fn increment(&mut self) {
		self.turn.increment(self.number_of_players());
	}
	pub fn decks(&self) -> [&Deck; 5] {
		[
			&self.draw,
			&self.discard,
			&self.monsters,
			&self.leaders,
			&self.next_monsters,
		]
	}
	pub fn decks_mut(&mut self) -> [&mut Deck; 5] {
		[
			&mut self.draw,
			&mut self.discard,
			&mut self.monsters,
			&mut self.leaders,
			&mut self.next_monsters,
		]
	}

	pub fn number_of_players(&self) -> usize {
		self.players.len()
	}

	pub fn new() -> Self {
		Game {
			// card_specs: specification::get_card_specs(),
			players: Default::default(),
			showdown: Default::default(),
			draw: Deck::new(DeckSpec {
				visibility: VisibilitySpec::summary(),
				path: DeckPath::Draw,
			}),
			discard: Deck::new(DeckSpec {
				visibility: VisibilitySpec::summary(),
				path: DeckPath::Discard,
			}),
			monsters: Deck::new(DeckSpec {
				visibility: VisibilitySpec::visible(),
				path: DeckPath::ActiveMonsters,
			}),
			leaders: Deck::new(DeckSpec {
				visibility: VisibilitySpec::invisible(),
				path: DeckPath::PartyLeaders,
			}),
			next_monsters: Deck::new(DeckSpec {
				visibility: VisibilitySpec::summary(),
				path: DeckPath::NextMonsters,
			}),
			turn: Default::default(),
		}
	}

	// Maybe cards should have been a top level field on game?
	// The individual decks could just have card ids in them...
	pub fn maybe_card(&self, card_path: CardPath) -> Option<&Card> {
		match card_path {
			CardPath::TopCardIn(deck_path, card_id) => self.deck(deck_path).card(card_id),
			CardPath::ModifyingCardIn(deck_path, top_card_id, modifier_card_id) => {
				self.deck(deck_path).modifier(top_card_id, modifier_card_id)
			}
			CardPath::Leader(player_index, _) => Some(&self.players[player_index].leader),
		}
		// self.deck(card_path.deck_path()).stack(card_path.top_id())
	}
	pub fn card(&self, card_path: CardPath) -> &Card {
		if let Some(card) = self.maybe_card(card_path) {
			card
		} else {
			log::info!("Unable to find card at card path {:?}", card_path);
			unreachable!()
		}
	}

	// Maybe cards should have been a top level field on game?
	// The individual decks could just have card ids in them...
	pub fn find_card(&self, card_id: ids::CardId) -> Option<&Card> {
		for deck in self.decks().iter() {
			if let Some(card) = deck.card(card_id) {
				return Some(card);
			}
		}
		for player in self.players.iter() {
			if player.leader.id == card_id {
				return Some(&player.leader);
			}
			for deck in player.decks().iter() {
				if let Some(card) = deck.card(card_id) {
					return Some(card);
				}
			}
		}
		None
	}

	pub fn current_player(&self) -> &Player {
		&self.players[self.turn.active_player_index()]
	}

	pub fn current_player_mut(&mut self) -> &mut Player {
		&mut self.players[self.turn.active_player_index()]
	}

	pub fn take_current_task(
		&mut self,
		player_index: ids::PlayerIndex,
	) -> Option<Box<dyn PlayerTask>> {
		self.players[player_index].take_current_task()
		// None
	}

	pub fn deck(&self, deck_path: DeckPath) -> &Deck {
		match deck_path {
			DeckPath::Draw => &self.draw,
			DeckPath::Discard => &self.discard,
			DeckPath::PartyLeaders => &self.leaders,
			DeckPath::ActiveMonsters => &self.monsters,
			DeckPath::NextMonsters => &self.next_monsters,
			DeckPath::Hand(index) => &self.players[index].hand,
			DeckPath::Party(index) => &self.players[index].party,
			DeckPath::SlainMonsters(index) => &self.players[index].slain_monsters,
		}
	}
	pub fn deck_mut(&mut self, deck_path: DeckPath) -> &mut Deck {
		match deck_path {
			DeckPath::Draw => &mut self.draw,
			DeckPath::Discard => &mut self.discard,
			DeckPath::PartyLeaders => &mut self.leaders,
			DeckPath::ActiveMonsters => &mut self.monsters,
			DeckPath::NextMonsters => &mut self.next_monsters,
			DeckPath::Hand(index) => &mut self.players[index].hand,
			DeckPath::Party(index) => &mut self.players[index].party,
			DeckPath::SlainMonsters(index) => &mut self.players[index].slain_monsters,
		}
	}

	pub fn move_card(
		&mut self,
		source: DeckPath,
		destination: DeckPath,
		card_id: ids::CardId,
	) -> SlayResult<()> {
		let stack = self.deck_mut(source).take_card(card_id)?;
		self.deck_mut(destination).add(stack);
		Ok(())
	}

	// pub(crate) fn get_player_name(&self, player_index: ids::PlayerIndex) -> String {
	// 	self.players[player_index].name.to_owned()
	// }

	pub(crate) fn set_active_player(&mut self, player_index: ids::PlayerIndex) {
		self.turn.set_active_player(player_index);
	}

	pub(crate) fn get_turn(&self) -> Turn {
		self.turn.to_owned()
	}

	pub(crate) fn replentish_for(&mut self, number_to_draw: usize) {
		if self.draw.num_top_cards() >= number_to_draw {
			return;
		}
		self.draw.extend(self.discard.drain(..));
	}

	pub(crate) fn player_param(
		&self,
		player_index: ids::PlayerIndex,
		param: &TaskParamName,
	) -> SlayResult<ids::PlayerIndex> {
		self.players[player_index]
			.tasks
			.get_player_value(param)
			.ok_or_else(|| SlayError::n(format!("Missing required player parameter: {:?}", param)))
	}
	pub(crate) fn card_param(
		&self,
		player_index: ids::PlayerIndex,
		param: &TaskParamName,
	) -> SlayResult<Option<ids::CardId>> {
		self.players[player_index]
			.tasks
			.get_card_value(param)
			.ok_or_else(|| SlayError::n(format!("Missing required card parameter: {:?}", param)))
	}

	pub(crate) fn players_with_stacks(&self) -> Vec<ids::PlayerIndex> {
		self
			.players
			.iter()
			.filter(|player| player.party.num_top_cards() > 0)
			.map(|player| player.player_index)
			.collect()
	}

	pub(crate) fn was_card_played(&self, player_index: Option<usize>, card_id: ids::CardId) -> bool {
		if let Some(player_index) = player_index {
			self.players[player_index].was_card_played(&card_id)
		} else {
			false
		}
	}
}

impl Default for Game {
	fn default() -> Self {
		Self::new()
	}
}

// Summarize the roll...
impl Summarizable for Game {
	fn summarize<W: Write>(
		&self,
		f: &mut BufWriter<W>,
		indentation_level: u32,
	) -> Result<(), std::io::Error> {
		for _ in 0..indentation_level {
			write!(f, "  ")?;
		}
		writeln!(f, "players:")?;
		for player in self.players.iter() {
			player.summarize(f, indentation_level + 1)?;
		}
		self.discard.summarize(f, indentation_level + 1)?;
		self.monsters.summarize(f, indentation_level + 1)?;
		self.draw.summarize(f, indentation_level + 1)?;
		self.next_monsters.summarize(f, indentation_level + 1)?;

		Ok(())
	}
}

pub fn get_perspective(
	owner_id: ids::PlayerIndex,
	viewer_id: Option<ids::PlayerIndex>,
) -> &'static Perspective {
	if let Some(player_id) = viewer_id {
		if owner_id == player_id {
			&Perspective::Owner
		} else {
			&Perspective::Spectator
		}
	} else {
		&Perspective::Spectator
	}
}

impl Game {
	pub fn to_statics(&self, player_index: ids::PlayerIndex) -> GameStaticInformation {
		GameStaticInformation {
			player_index,
			players: self
				.players
				.iter()
				.map(|p| PlayerStaticInformation {
					name: p.name.to_owned(),
					leader: p.leader.to_owned(),
				})
				.collect(),
		}
	}
	pub fn to_player_perspective(&self, viewing_player: Option<ids::PlayerIndex>) -> GamePerspective {
		let choices = &if let Some(player_index) = viewing_player {
			self.players[player_index].choices.as_ref()
		} else {
			None
		};
		GamePerspective {
			players: self
				.players
				.iter()
				.map(|p| p.to_perspective(self, get_perspective(p.player_index, viewing_player)))
				.collect(),
			decks: self
				.decks()
				.iter()
				.filter(|d| d.visible_to_spectator())
				.map(|d| d.to_spectator_perspective(self, None))
				.collect(),
			turn: self.get_turn(),
			choices: choices.map(|c| c.to_perspective()),
			roll: self.showdown.get_roll().map(|r| r.to_perspective()),
			offer: self.showdown.get_offer().map(|o| o.to_perspective()),
			challenge: self.showdown.get_challenge().map(|o| o.to_perspective()),
		}
	}
}

impl GameStaticInformation {
	pub fn player_name(&self, player_index: ids::PlayerIndex) -> &String {
		&self.players[player_index].name
	}
}
