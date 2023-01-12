// use super::ids::{CardId, ChallengeId, ChoiceId, DeckId, ElementId, IdGenerator, PlayerId, RollId};

use crate::slay::choices::CardPath;
use crate::slay::choices::ChoicePerspective;
use crate::slay::choices::ChoicesPerspective;
use crate::slay::choices::DisplayPath;
use crate::slay::errors;
use crate::slay::errors::SlayResult;
use crate::slay::ids;
use crate::slay::specification::HeroType;
use crate::slay::specs::visibility::Perspective;
use crate::slay::specs::visibility::Visibility;
use crate::slay::specs::visibility::VisibilitySpec;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::state::stack::Stack;
use crate::slay::state::summarizable::Summarizable;

use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;
use std::iter::Iterator;
use std::ops::RangeBounds;

use super::player::HeroTypeCounter;
use super::stack::StackPerspective;

// Move this to the decks file?
#[derive(Debug, Clone)]
pub struct DeckSpec {
	pub visibility: VisibilitySpec,
	pub path: DeckPath,
}

// Lol, tried of looking for the deck by id...
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeckPath {
	Draw,
	Discard,
	PartyLeaders,
	ActiveMonsters,
	NextMonsters,
	Hand(ids::PlayerIndex),
	Party(ids::PlayerIndex),
	SlainMonsters(ids::PlayerIndex),
}

impl DeckPath {
	pub fn display(&self) -> DisplayPath {
		DisplayPath::DeckAt(*self)
	}

	pub fn get_player_index(&self) -> Option<ids::PlayerIndex> {
		match self {
			DeckPath::Draw => None,
			DeckPath::Discard => None,
			DeckPath::PartyLeaders => None,
			DeckPath::ActiveMonsters => None,
			DeckPath::NextMonsters => None,
			DeckPath::Hand(player_index) => Some(*player_index),
			DeckPath::Party(player_index) => Some(*player_index),
			DeckPath::SlainMonsters(player_index) => Some(*player_index),
		}
	}

	pub fn get_label(&self) -> String {
		match self {
			DeckPath::Draw => "Draw pile".to_string(),
			DeckPath::Discard => "Discard pile".to_string(),
			DeckPath::PartyLeaders => "Unused Party leaders".to_string(),
			DeckPath::ActiveMonsters => "Monsters".to_string(),
			DeckPath::NextMonsters => "Next monsters".to_string(),
			DeckPath::Hand(player_index) => format!("Player {}'s hand", player_index),
			DeckPath::Party(player_index) => format!("Player {}'s party", player_index),
			DeckPath::SlainMonsters(player_index) => format!("Player {}'s monsters", player_index),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PartialDeckPath {
	// Draw,
	// Discard,
	// PartyLeaders,
	// ActiveMonsters,
	// NextMonsters,
	Hand,
	Party,
	SlainMonsters,
}

impl PartialDeckPath {
	pub fn to_deck_path(&self, player_index: ids::PlayerIndex) -> DeckPath {
		match self {
			// PartialDeckPath::Draw => DeckPath::Draw,
			// PartialDeckPath::Discard => DeckPath::Discard,
			// PartialDeckPath::PartyLeaders => DeckPath::PartyLeaders,
			// PartialDeckPath::ActiveMonsters => DeckPath::ActiveMonsters,
			// PartialDeckPath::NextMonsters => DeckPath::NextMonsters,
			PartialDeckPath::Hand => DeckPath::Hand(player_index),
			PartialDeckPath::Party => DeckPath::Party(player_index),
			PartialDeckPath::SlainMonsters => DeckPath::SlainMonsters(player_index),
		}
	}
}

// pub const DRAW: DeckPath = DeckPath::GlobalDeck(GlobalDeckName::Draw);
// pub const DISCARD: DeckPath = DeckPath::GlobalDeck(GlobalDeckName::Discard);
// pub const LEADERS: DeckPath = DeckPath::GlobalDeck(GlobalDeckName::PartyLeaders);
// pub const MONSTERS: DeckPath = DeckPath::GlobalDeck(GlobalDeckName::ActiveMonsters);
// pub const NEXT_MONSTERS: DeckPath = DeckPath::GlobalDeck(GlobalDeckName::NextMonsters);
// impl DeckPath {
//     pub fn hand(player_index: ids::PlayerIndex) -> DeckPath { DeckPath::PlayerDeck(player_index, PlayerDeckName::Hand)}
//     pub fn party(player_index: ids::PlayerIndex) -> DeckPath { DeckPath::PlayerDeck(player_index, PlayerDeckName::Party)}
//     pub fn monsters(player_index: ids::PlayerIndex) -> DeckPath { DeckPath::PlayerDeck(player_index, PlayerDeckName::SlainMonsters)}
// }

#[derive(Debug, Clone)]
pub struct Deck {
	// TODO: hide internals...
	// TODO: remove the id...
	// TODO: make stacks optional...
	// TODO: Make the stacks just have a card id
	stacks: VecDeque<Stack>,
	pub spec: DeckSpec,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeckPerspective {
	pub count: usize,
	pub path: DeckPath,
	pub stacks: Option<Vec<StackPerspective>>,
}

impl Deck {
	pub fn new(spec: DeckSpec) -> Self {
		Self {
			stacks: Default::default(),
			spec,
		}
	}

	pub fn to_perspective(
		&self,
		game: &Game,
		player_index: Option<ids::PlayerIndex>,
		perspective: &Perspective,
	) -> DeckPerspective {
		let visibility = self.spec.visibility.get(perspective);
		match visibility {
			Visibility::Visible => DeckPerspective {
				count: self.num_top_cards(),
				path: self.spec.path,
				stacks: Some(
					self
						.stacks()
						.map(|s| s.to_perspective(game, player_index))
						.collect(),
				),
			},
			Visibility::Summary => DeckPerspective {
				count: self.num_top_cards(),
				path: self.spec.path,
				stacks: None,
			},
			Visibility::Invisible => {
				unreachable!();
			}
		}
	}

	pub fn to_spectator_perspective(
		&self,
		game: &Game,
		player_index: Option<ids::PlayerIndex>,
	) -> DeckPerspective {
		self.to_perspective(game, player_index, &Perspective::Spectator)
	}

	pub fn num_top_cards(&self) -> usize {
		self.stacks.len()
	}

	// pub fn list_top_cards_by_type(&self, card_type: &CardType) -> Vec<ids::CardId> {
	// 	self
	// 		.stacks
	// 		.iter()
	// 		.filter(|s| s.top.card_type() == card_type)
	// 		.map(|s| s.top.id)
	// 		.collect()
	// }

	pub fn stacks(&self) -> impl Iterator<Item = &Stack> {
		self.stacks.iter()
	}
	pub fn stacks_mut(&mut self) -> impl Iterator<Item = &mut Stack> {
		self.stacks.iter_mut()
	}
	// pub fn iter(&self) -> impl Iterator<Item = &Stack> {
	// 	self.stacks.iter()
	// }
	pub fn tops(&self) -> impl Iterator<Item = &Card> {
		self.stacks.iter().map(|stack| &stack.top)
	}
	// TODO: understand the whole '_ thing...
	pub fn top_paths(&self) -> impl Iterator<Item = CardPath> + '_ {
		self
			.tops()
			.map(|card| CardPath::TopCardIn(self.spec.path, card.id))
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

	pub fn is_visible(&self, perspective: &Perspective) -> bool {
		self.spec.visibility.is_visible(perspective)
	}

	pub fn visible_to_spectator(&self) -> bool {
		self.is_visible(&Perspective::Spectator)
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

	pub fn count_hero_types(&self, hero_types: &mut HeroTypeCounter) {
		for stack in self.stacks.iter() {
			hero_types.maybe_add_hero_type(stack.get_hero_type());
		}
	}
	pub fn collect_hero_types(&self, hero_types: &mut HashSet<HeroType>) {
		hero_types.extend(self.stacks.iter().flat_map(|stack| stack.get_hero_type()))
	}
	pub(crate) fn contains_hero_type(&self, hero_type: &HeroType) -> bool {
		self.stacks.iter().any(|stack| match stack.get_hero_type() {
			Some(ht) => ht == *hero_type,
			None => false,
		})
	}

	pub fn take(&mut self, card_id: ids::CardId) -> Option<Stack> {
		if let Some(position) = self.stacks.iter().position(|s| s.top.id == card_id) {
			// Take the whole stack
			return self.stacks.remove(position);
		}

		for stack in self.stacks.iter_mut() {
			if let Some(position) = stack.modifiers.iter().position(|c| c.id == card_id) {
				// Just the modifier
				return Some(Stack::new(stack.modifiers.remove(position)));
			}
		}
		None

		// #[derive(Debug, Clone)]
		// pub enum OnIsModifier {
		// 	TakeTheModifier,
		// 	TakeTheWholeStack,
		// }

		// The other implementation:
		// self
		// 	.stacks
		// 	.iter()
		// 	.position(|s| s.contains(card_id))
		// 	.and_then(|i| self.stacks.remove(i))
	}

	pub fn take_at_index(&mut self, index: usize) -> Stack {
		self.stacks.remove(index).unwrap()
	}

	pub fn take_card(&mut self, card_id: ids::CardId) -> SlayResult<Stack> {
		self
			.take(card_id)
			.ok_or_else(|| errors::SlayError::new("Unable to find card in deck."))
	}

	pub fn maybe_deal(&mut self) -> Option<Stack> {
		self.stacks.pop_front()
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

	// pub(crate) fn other_cards(&self, exclude: &HashSet<ids::CardId>) -> HashSet<ids::CardId> {
	// 	self
	// 		.stacks
	// 		.iter()
	// 		.filter(|stack| exclude.contains(&stack.top.id))
	// 		.map(|stack| stack.top.id)
	// 		.collect()
	// }

	pub(crate) fn modifier(&self, top_card_id: u32, modifier_card_id: u32) -> Option<&Card> {
		for stack in self.stacks.iter() {
			if stack.top.id != top_card_id {
				continue;
			}
			for modifier in stack.modifiers.iter() {
				if modifier.id == modifier_card_id {
					return Some(modifier);
				}
			}
		}
		None
	}
}

impl Summarizable for Deck {
	fn summarize<W: Write>(
		&self,
		f: &mut BufWriter<W>,
		indentation_level: u32,
	) -> Result<(), std::io::Error> {
		for _ in 0..indentation_level {
			write!(f, "  ")?;
		}
		write!(f, "{}: ", self.spec.path.get_label())?;
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
		writeln!(f)?;
		Ok(())
	}
}

impl DeckPerspective {
	pub fn choices(&self, choices: &Option<ChoicesPerspective>) -> Vec<ChoicePerspective> {
		if let Some(choices) = choices {
			choices.represented_by(&DisplayPath::DeckAt(self.path))
		} else {
			Vec::new()
		}
	}
}
