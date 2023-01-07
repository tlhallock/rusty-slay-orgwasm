use crate::slay::choices::ChoicesPerspective;
use crate::slay::choices::DisplayPath;
use crate::slay::errors;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::modifiers;
use crate::slay::showdown::challenge::ChallengePerspective;
use crate::slay::showdown::current_showdown::CurrentShowdown;
use crate::slay::showdown::offer::OfferChallengesPerspective;
use crate::slay::showdown::roll_state::RollPerspective;
use crate::slay::specification;
use crate::slay::specification::DeckSpec;
use crate::slay::state::deck::Deck;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::deck::DeckPerspective;
use crate::slay::state::player::Player;
use crate::slay::state::player::PlayerPerspective;
use crate::slay::state::stack::Card;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::tasks::PlayerTask;
use crate::slay::visibility::Perspective;
use crate::slay::visibility::VisibilitySpec;

use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;
use std::iter::Iterator;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Turn {
	turn_number: u32,
	round_number: u32,
	player_index: u32,
}

impl Turn {
	pub(crate) fn set_active_player(&mut self, player_index: ids::PlayerIndex) {
		self.player_index = player_index as u32;
	}
	pub fn still_active(&self, duration: &modifiers::ModifierDuration) -> bool {
		match duration {
			modifiers::ModifierDuration::Forever => true,
			modifiers::ModifierDuration::UntilTurn(t, p) => {
				self.round_number <= *t && self.player_index <= *p
			}
		}
	}

	fn increment(&mut self, number_of_players: usize) {
		self.player_index += 1;
		self.turn_number += 1;
		if self.player_index < number_of_players as u32 {
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

impl Game {
	pub fn get_element_id(&self, display_path: &Option<DisplayPath>) -> Option<ids::ElementId> {
		display_path.as_ref().and_then(|p| match p {
			DisplayPath::DeckAt(d) => Some(self.deck(*d).id),
			DisplayPath::CardIn(_, id) => Some(*id),
			DisplayPath::Player(player_index) => Some(self.players[*player_index].id),
			DisplayPath::Roll(_) => None,
		})
	}

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

	pub fn new(context: &mut GameBookKeeping) -> Self {
		Game {
			// card_specs: specification::get_card_specs(),
			players: Default::default(),
			showdown: Default::default(),
			draw: Deck::new(
				context.id_generator.generate(),
				DeckSpec {
					visibility: VisibilitySpec::summary(),
					label: "Draw pile".to_string(),
				},
			),
			discard: Deck::new(
				context.id_generator.generate(),
				DeckSpec {
					visibility: VisibilitySpec::summary(),
					label: "Discard pile".to_string(),
				},
			),
			monsters: Deck::new(
				context.id_generator.generate(),
				DeckSpec {
					visibility: VisibilitySpec::visible(),
					label: "Active monsters".to_string(),
				},
			),
			leaders: Deck::new(
				context.id_generator.generate(),
				DeckSpec {
					visibility: VisibilitySpec::invisible(),
					label: "Party leaders".to_string(),
				},
			),
			next_monsters: Deck::new(
				context.id_generator.generate(),
				DeckSpec {
					visibility: VisibilitySpec::summary(),
					label: "Upcoming monsters".to_string(),
				},
			),
			turn: Default::default(),
		}
	}

	// Maybe cards should have been a top level field on game?
	// The individual decks could just have card ids in them...
	pub fn card(&self, card_id: ids::CardId) -> Option<&Card> {
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

	pub fn player(&mut self, player_id: ids::PlayerId) -> SlayResult<&mut Player> {
		self
			.players
			.iter_mut()
			.find(|p| p.id == player_id)
			.ok_or_else(|| errors::SlayError::new("Could not find player"))
	}
	pub fn player_index(&self, player_id: ids::PlayerId) -> Option<ids::PlayerIndex> {
		// unreachable!()?
		self.players.iter().position(|p| p.id == player_id)
	}

	pub fn current_player(&self) -> &Player {
		&self.players[self.turn.active_player_index()]
	}

	pub fn current_player_mut(&mut self) -> &mut Player {
		&mut self.players[self.turn.active_player_index()]
	}

	pub fn take_current_task(
		&mut self, player_index: ids::PlayerIndex,
	) -> Option<Box<dyn PlayerTask>> {
		self.players[player_index].take_current_task()
		// None
	}

	// pub(crate) fn get_completion_tracker(
	//     &self,
	//     path: rolls::CompletionPath,
	// ) -> SlayResult<&rolls::CompletionTracker> {
	//     match path {
	//         rolls::CompletionPath::Roll => Ok(&self
	//             .roll
	//             .as_ref()
	//             .ok_or_else(|| SlayError::new("No active roll"))?
	//             .completion_tracker),
	//         rolls::CompletionPath::OfferChallenges => Ok(&self
	//             .challenges_offer
	//             .as_ref()
	//             .ok_or_else(|| SlayError::new("No active challenge offers"))?
	//             .completion_tracker),
	//         rolls::CompletionPath::Challege => Ok(&self
	//             .challenge
	//             .as_ref()
	//             .ok_or_else(|| SlayError::new("No active challenge"))?
	//             .completion_tracker),
	//     }
	// }

	// pub(crate) fn get_completion_tracker_mut(
	//     &mut self,
	//     path: rolls::CompletionPath,
	// ) -> SlayResult<&mut rolls::CompletionTracker> {
	//     match path {
	//         rolls::CompletionPath::Roll => Ok(&mut self
	//             .roll
	//             .as_mut()
	//             .ok_or_else(|| SlayError::new("No active roll"))?
	//             .completion_tracker),
	//         rolls::CompletionPath::OfferChallenges => Ok(&mut self
	//             .challenges_offer
	//             .as_mut()
	//             .ok_or_else(|| SlayError::new("No active challenge offers"))?
	//             .completion_tracker),
	//         rolls::CompletionPath::Challege => Ok(&mut self
	//             .challenge
	//             .as_mut()
	//             .ok_or_else(|| SlayError::new("No active challenge"))?
	//             .completion_tracker),
	//     }
	// }

	pub fn deck(&self, path: DeckPath) -> &Deck {
		match path {
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
	pub fn deck_mut(&mut self, path: DeckPath) -> &mut Deck {
		match path {
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
		&mut self, source: DeckPath, destination: DeckPath, card_id: ids::CardId,
	) -> SlayResult<()> {
		let stack = self.deck_mut(source).take_card(card_id)?;
		self.deck_mut(destination).add(stack);
		Ok(())
	}

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
}

// Summarize the roll...
impl Summarizable for Game {
	fn summarize<W: Write>(
		&self, f: &mut BufWriter<W>, indentation_level: u32,
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

pub fn get_perspective(owner_id: ids::PlayerId, player_id: ids::PlayerId) -> &'static Perspective {
	if owner_id == player_id {
		&Perspective::Owner
	} else {
		&Perspective::Spectator
	}
}

impl Game {
	pub fn to_player_perspective(&self, player_id: ids::PlayerId) -> GamePerspective {
		let perspective = &Perspective::Spectator;
		let choices = &self
			.players
			.iter()
			.find(|p| p.id == player_id)
			.and_then(|p| p.choices.as_ref())
			.map(|c| c.to_perspective(self));
		let players = self
			.players
			.iter()
			.map(|p| {
				p.to_perspective(
					get_perspective(p.id, player_id),
					choices,
					p.player_index == self.active_player_index(),
				)
			})
			.collect();
		let decks = self
			.decks()
			.iter()
			.filter(|d| d.is_visible(perspective))
			.map(|d| d.to_perspective(perspective, None, choices))
			.collect();
		GamePerspective {
			players,
			decks,
			turn: self.get_turn(),
			choices: choices.to_owned(), // TODO
			roll: self
				.showdown
				.get_roll()
				.map(|r| r.to_perspective(self, choices)),
			offer: self
				.showdown
				.get_offer()
				.map(|o| o.to_perspective(self, choices)),
			challenge: self
				.showdown
				.get_challenge()
				.map(|o| o.to_perspective(self, choices)),
		}
	}
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
	pub fn rotated_players(&self) -> Vec<&PlayerPerspective> {
		let mut ret: Vec<&PlayerPerspective> = self.players.iter().collect();
		let position = ret.iter().position(|p| p.me);
		if let Some(index) = position {
			ret.rotate_left(index)
		}
		ret
	}
}
