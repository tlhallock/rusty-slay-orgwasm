// lol, I can't say `use crate::ids;`

use super::choices::DisplayPath;
// use super::ids::{CardId, ChallengeId, ChoiceId, DeckId, ElementId, IdGenerator, PlayerId, RollId};
use super::ids;
use super::modifiers::PlayerBuffs;
use super::specification;
use super::specification::CardSpec;
use super::specification::CardType;
use super::tasks::PlayerTask;
use super::tasks::PlayerTasks;
use crate::slay::choices;
use crate::slay::errors;
use crate::slay::game_context;
use crate::slay::modifiers;
use crate::slay::showdown::current_showdown::CurrentShowdown;
use std::io::Write;

use crate::slay::specification::HeroType;
use crate::slay::tasks;

use errors::SlayResult;

use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufWriter;
use std::ops::RangeBounds;

use std::io::Result as IoResult;
use std::iter::Iterator;

#[derive(Debug, Clone, Copy)]
pub enum ChoiceParamType {
	Player,
	Card,
	Enumeration,
	Index,
}

#[derive(Debug, Clone)]
pub struct Card {
	pub id: ids::CardId,
	pub spec: CardSpec,
}

impl Card {
	pub fn new(id: ids::CardId, spec: CardSpec) -> Self {
		Card { id, spec }
	}

	pub fn modification_amounts(&self) -> Vec<i32> {
		self.spec.modifiers.iter().map(|x| *x).collect() // ::<Vec<(ids::CardId, i32)>>()
	}

	pub fn label(&self) -> String {
		self.spec.label.to_string()
	}

	pub fn monster_spec(&self) -> &Option<specification::MonsterSpec> {
		&self.spec.monster
	}

	pub fn hero_ability(&self) -> &Option<specification::HeroAbility> {
		&self.spec.hero_ability
	}

	pub fn card_type(&self) -> &specification::CardType {
		&self.spec.card_type
	}

	pub fn get_hero_type(&self) -> Option<specification::HeroType> {
		match &self.spec.card_type {
			specification::CardType::Hero(hero_type)
			| specification::CardType::PartyLeader(hero_type) => Some(*hero_type),
			_ => None,
		}
	}
}

#[derive(Debug, Clone)]
pub struct Stack {
	// pub id: ElementId,
	pub top: Card,
	pub modifiers: Vec<Card>,
}

impl Stack {
	pub fn new(top: Card) -> Self {
		Self {
			top,
			modifiers: Vec::new(),
		}
	}

	pub fn get_hero_type(&self) -> Option<specification::HeroType> {
		self.top.get_hero_type()
	}

	pub fn contains(&self, card_id: ids::CardId) -> bool {
		self.top.id == card_id || self.modifiers.iter().any(|c| c.id == card_id)
	}
}

// Lol, tried of looking for the deck by id...
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeckPath {
	Draw,
	Discard,
	PartyLeaders,
	ActiveMonsters,
	NextMonsters,
	Hand(usize),
	Party(usize),
	SlainMonsters(usize),
}

// pub const DRAW: DeckPath = DeckPath::GlobalDeck(GlobalDeckName::Draw);
// pub const DISCARD: DeckPath = DeckPath::GlobalDeck(GlobalDeckName::Discard);
// pub const LEADERS: DeckPath = DeckPath::GlobalDeck(GlobalDeckName::PartyLeaders);
// pub const MONSTERS: DeckPath = DeckPath::GlobalDeck(GlobalDeckName::ActiveMonsters);
// pub const NEXT_MONSTERS: DeckPath = DeckPath::GlobalDeck(GlobalDeckName::NextMonsters);
// impl DeckPath {
//     pub fn hand(player_index: usize) -> DeckPath { DeckPath::PlayerDeck(player_index, PlayerDeckName::Hand)}
//     pub fn party(player_index: usize) -> DeckPath { DeckPath::PlayerDeck(player_index, PlayerDeckName::Party)}
//     pub fn monsters(player_index: usize) -> DeckPath { DeckPath::PlayerDeck(player_index, PlayerDeckName::SlainMonsters)}
// }

#[derive(Debug, Clone)]
pub struct Deck {
	// TODO: hide internals...
	pub id: ids::DeckId,
	stacks: VecDeque<Stack>,
	pub spec: specification::DeckSpec,
}

impl Deck {
	pub fn new(id_gen: &mut ids::IdGenerator, spec: specification::DeckSpec) -> Self {
		Self {
			id: id_gen.generate(),
			stacks: Default::default(),
			spec,
		}
	}

	pub fn num_top_cards(&self) -> usize {
		self.stacks.len()
	}

	pub fn list_top_cards_by_type(&self, card_type: &CardType) -> Vec<ids::CardId> {
		self
			.stacks
			.iter()
			.filter(|s| s.top.card_type() == card_type)
			.map(|s| s.top.id)
			.collect()
	}

	pub fn iter(&self) -> impl Iterator<Item = &Stack> {
		self.stacks.iter()
	}

	pub fn extend<D: IntoIterator<Item = Stack>>(&mut self, drained: D) {
		self.stacks.extend(drained);
	}

	// Wish the return type didn't have to expose the inner data type...
	pub fn drain<R>(&mut self, range: R) -> std::collections::vec_deque::Drain<Stack>
	where
		R: RangeBounds<usize>,
	{
		self.stacks.drain(range)
	}

	pub fn is_visible(&self, perspective: &specification::Perspective) -> bool {
		self.spec.visibility.is_visible(perspective)
	}

	pub fn add(&mut self, stack: Stack) {
		self.stacks.push_back(stack);
	}

	pub fn add_card(&mut self, c: Card) {
		self.stacks.push_back(Stack {
			top: c,
			modifiers: vec![],
		});
	}

	pub fn hero_types(&self) -> HashSet<HeroType> {
		return self
			.stacks
			.iter()
			.filter_map(|s| s.get_hero_type())
			.collect();
	}

	pub fn take(&mut self, card_id: ids::CardId) -> Option<Stack> {
		self
			.stacks
			.iter()
			.position(|s| s.contains(card_id))
			.and_then(|i| self.stacks.remove(i))
	}

	pub fn take_card(&mut self, card_id: ids::CardId) -> SlayResult<Stack> {
		self
			.take(card_id)
			.ok_or_else(|| errors::SlayError::new("Unable to find card in deck."))
	}

	pub fn deal(&mut self) -> Stack {
		self.stacks.pop_front().unwrap()
		// // TODO:
		// let mut stack = self.stacks.pop().unwrap();
		// if stack.cards.len() != 1 {
		//     println!("Losing cards.");
		// }
		// return stack.cards.pop().unwrap();

		// let mut cards = &mut self.stacks.pop()?.cards;
		// if cards.len() != 1 {
		//     println!("Losing cards.");
		//     return None;
		// }
		// return Some(&mut cards[0]);
	}

	pub(crate) fn card(&self, card_id: u32) -> Option<&Card> {
		self
			.stacks
			.iter()
			.find(|s| s.top.id == card_id)
			.map(|s| &s.top)
	}
	pub(crate) fn card_mut(&mut self, card_id: u32) -> Option<&mut Card> {
		self
			.stacks
			.iter_mut()
			.find(|s| s.top.id == card_id)
			.map(|s| &mut s.top)
	}
}

// Split into choice type?
// #[derive(Debug, Clone)]
// pub enum Action {
//     ReplaceHand,
//     DrawCard,
//     Forfeit,

//     Attack(ids::CardId),
//     PlaceCard(ids::CardId),
//     CastMagic(ids::CardId),
//     UseHero(ids::CardId),
// }

#[derive(Clone, Debug)]
pub struct Player {
	pub id: ids::PlayerId,
	pub player_index: usize,
	pub name: String,

	pub buffs: PlayerBuffs,
	pub choices: Option<choices::Choices>,
	pub tasks: PlayerTasks,

	pub leader: Card,

	pub hand: Deck,
	pub party: Deck,
	pub slain_monsters: Deck,

	played_this_turn: HashSet<ids::CardId>,
	remaining_action_points: u32,
	// current modifiers
	// player information
}

impl Player {
	pub fn put_current_task_back(
		&mut self, task: Box<dyn tasks::PlayerTask>,
	) -> errors::SlayResult<()> {
		self.tasks.put_current_task_back(task)?;
		Ok(())
	}
	pub fn decks(&self) -> [&Deck; 3] {
		[&self.hand, &self.party, &self.slain_monsters]
	}
	pub fn decks_mut(&mut self) -> [&mut Deck; 3] {
		[&mut self.hand, &mut self.party, &mut self.slain_monsters]
	}
	pub fn new(
		id_gen: &mut ids::IdGenerator, name: String, player_index: usize, leader: Card,
	) -> Self {
		Player {
			id: id_gen.generate(),
			player_index,
			name,
			choices: None,
			tasks: Default::default(),
			remaining_action_points: 0,
			leader,
			buffs: Default::default(),
			hand: Deck {
				id: id_gen.generate(),
				stacks: Default::default(),
				spec: specification::DeckSpec {
					visibility: specification::VisibilitySpec::summary(),
					label: "Hand".to_string(),
				},
			},
			party: Deck {
				id: id_gen.generate(),
				stacks: Default::default(),
				spec: specification::DeckSpec {
					visibility: specification::VisibilitySpec::visible(),
					label: "Party".to_string(),
				},
			},
			slain_monsters: Deck {
				id: id_gen.generate(),
				stacks: Default::default(),
				spec: specification::DeckSpec {
					visibility: specification::VisibilitySpec::visible(),
					label: "Slain monsters".to_string(),
				},
			},
			played_this_turn: Default::default(),
		}
	}

	pub fn turn_begin(&mut self) {
		self.remaining_action_points = 3;
	}

	pub fn action_points_used(&mut self, amount: u32) {
		self.remaining_action_points -= amount;
	}

	pub fn set_card_played(&mut self, card_id: ids::CardId) {
		self.played_this_turn.insert(card_id);
	}

	pub fn was_card_played(&self, card_id: &ids::CardId) -> bool {
		self.played_this_turn.contains(card_id)
	}

	pub fn turn_end(&mut self) {
		self.played_this_turn.clear();
	}

	pub fn hero_types(&self) -> HashSet<HeroType> {
		// Could this be a one liner?
		let mut hero_types = self.party.hero_types();
		hero_types.insert(self.leader.get_hero_type().unwrap());
		hero_types
	}

	pub fn take_current_task(&mut self) -> Option<Box<dyn tasks::PlayerTask>> {
		self.tasks.take_current_task()
	}

	fn clear_expired_modifiers(&mut self, turn: &Turn) {
		self.buffs.clear_expired_modifiers(turn);
	}

	pub(crate) fn get_remaining_action_points(&self) -> u32 {
		self.remaining_action_points
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Turn {
	turn_number: u32,
	round_number: u32,
	player_index: u32,
}

impl Turn {
	pub(crate) fn set_active_player(&mut self, player_index: usize) {
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
	pub fn active_player_index(&self) -> usize {
		self.player_index as usize
	}
}

#[derive(Clone, Debug)]
pub struct Game {
	pub players: Vec<Player>,

	pub showdown: CurrentShowdown,
	turn: Turn,

	// decks
	pub draw: Deck,
	pub discard: Deck,
	pub monsters: Deck,
	pub leaders: Deck,
	pub next_monsters: Deck,
}

impl Game {
	pub fn get_element_id(&self, display_path: &Option<DisplayPath>) -> Option<ids::ElementId> {
		display_path
			.as_ref()
			.map(|p| match p {
				DisplayPath::DeckAt(d) => Some(self.deck(*d).id),
				DisplayPath::CardIn(_, id) => Some(*id),
				DisplayPath::Player(player_index) => Some(self.players[*player_index].id),
				DisplayPath::Roll(_) => None,
			})
			.flatten()
	}

	pub fn clear_expired_modifiers(&mut self) {
		self
			.players
			.iter_mut()
			.for_each(|player| player.clear_expired_modifiers(&self.turn))
	}
	pub fn active_player_index(&self) -> usize {
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

	pub fn new(context: &mut game_context::GameBookKeeping) -> Self {
		Game {
			// card_specs: specification::get_card_specs(),
			players: Default::default(),
			showdown: Default::default(),
			draw: Deck {
				id: context.id_generator.generate(),
				stacks: Default::default(),
				spec: specification::DeckSpec {
					visibility: specification::VisibilitySpec::summary(),
					label: "Draw pile".to_string(),
				},
			},
			discard: Deck {
				id: context.id_generator.generate(),
				stacks: Default::default(),
				spec: specification::DeckSpec {
					visibility: specification::VisibilitySpec::summary(),
					label: "Discard pile".to_string(),
				},
			},
			monsters: Deck {
				id: context.id_generator.generate(),
				stacks: Default::default(),
				spec: specification::DeckSpec {
					visibility: specification::VisibilitySpec::visible(),
					label: "Active monsters".to_string(),
				},
			},
			leaders: Deck {
				id: context.id_generator.generate(),
				stacks: Default::default(),
				spec: specification::DeckSpec {
					visibility: specification::VisibilitySpec::invisible(),
					label: "Party leaders".to_string(),
				},
			},
			next_monsters: Deck {
				id: context.id_generator.generate(),
				stacks: Default::default(),
				spec: specification::DeckSpec {
					visibility: specification::VisibilitySpec::summary(),
					label: "Upcoming monsters".to_string(),
				},
			},
			turn: Default::default(),
		}
	}

	// Maybe cards should have been a top level field on game?
	// The individual decks could just have card ids in them...
	pub fn card(&self, card_id: ids::CardId) -> Option<&Card> {
		for deck in self.decks().iter() {
			if let Some(card) = deck.card(card_id) {
				return Some(&card);
			}
		}
		for player in self.players.iter() {
			if player.leader.id == card_id {
				return Some(&player.leader);
			}
			for deck in player.decks().iter() {
				if let Some(card) = deck.card(card_id) {
					return Some(&card);
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
	pub fn player_index(&self, player_id: ids::PlayerId) -> Option<usize> {
		self.players.iter().position(|p| p.id == player_id)
	}

	pub fn current_player(&self) -> &Player {
		&self.players[self.turn.active_player_index()]
	}

	pub fn current_player_mut(&mut self) -> &mut Player {
		&mut self.players[self.turn.active_player_index()]
	}

	pub fn take_current_task(&mut self, player_index: usize) -> Option<Box<dyn PlayerTask>> {
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
		self.deck_mut(destination).stacks.push_back(stack);
		Ok(())
	}

	pub(crate) fn set_active_player(&mut self, player_index: usize) {
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

pub trait Summarizable {
	fn summarize<W: Write>(
		&self, f: &mut BufWriter<W>, indentation_level: u32,
	) -> Result<(), std::io::Error>;
}

impl Summarizable for Card {
	fn summarize<W: Write>(
		&self, f: &mut BufWriter<W>, indentation_level: u32,
	) -> Result<(), std::io::Error> {
		write!(
			f,
			"({}) {}",
			self.id,
			self.spec.label,
			//  if self.played_this_turn { "X" } else { "" }
		)
	}
}
impl Summarizable for Stack {
	fn summarize<W: Write>(
		&self, f: &mut BufWriter<W>, indentation_level: u32,
	) -> Result<(), std::io::Error> {
		write!(f, "[")?;
		self.top.summarize(f, indentation_level + 1)?;
		write!(f, " {}], ", self.modifiers.len())
	}
}

impl Summarizable for Deck {
	fn summarize<W: Write>(
		&self, f: &mut BufWriter<W>, indentation_level: u32,
	) -> Result<(), std::io::Error> {
		for _ in 0..indentation_level {
			write!(f, "  ")?;
		}
		write!(f, "{}: ", self.spec.label)?;
		let num_stacks = self.stacks.len();
		if num_stacks > 8 {
			for stack in self.stacks.range(0..4) {
				stack.summarize(f, indentation_level + 1)?;
			}
			write!(f, "...  ")?;
			for stack in self.stacks.range((num_stacks - 4)..num_stacks) {
				stack.summarize(f, indentation_level + 1)?;
			}
		} else {
			for stack in self.stacks.iter() {
				stack.summarize(f, indentation_level + 1)?;
			}
		}
		write!(f, "\n")?;
		Ok(())
	}
}

impl Summarizable for Player {
	fn summarize<W: Write>(
		&self, f: &mut BufWriter<W>, indentation_level: u32,
	) -> Result<(), std::io::Error> {
		for _ in 0..indentation_level {
			write!(f, "  ")?;
		}
		writeln!(f, "player {}", self.player_index)?;
		self.hand.summarize(f, indentation_level + 1)?;
		self.party.summarize(f, indentation_level + 1)?;
		self.slain_monsters.summarize(f, indentation_level + 1)?;

		if let Some(choices) = self.choices.as_ref() {
			choices.summarize(f, indentation_level + 1)?;
		}
		self.tasks.summarize(f, indentation_level + 1)?;

		Ok(())
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
