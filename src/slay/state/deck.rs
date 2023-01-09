// use super::ids::{CardId, ChallengeId, ChoiceId, DeckId, ElementId, IdGenerator, PlayerId, RollId};

use crate::slay::choices::CardPath;
use crate::slay::choices::ChoiceAssociation;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayPath;
use crate::slay::errors;
use crate::slay::errors::SlayResult;
use crate::slay::ids;
use crate::slay::specification;
use crate::slay::specification::CardType;
use crate::slay::specification::DeckSpec;
use crate::slay::specification::HeroType;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::state::stack::Stack;
use crate::slay::state::stack::StackPerspective;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::visibility::Perspective;
use crate::slay::visibility::Visibility;

use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;
use std::iter::Iterator;
use std::ops::RangeBounds;

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
	pub spec: specification::DeckSpec,
}

impl Deck {
	pub fn new(spec: DeckSpec) -> Self {
		Self {
			stacks: Default::default(),
			spec,
		}
	}

	pub fn to_perspective(
		&self, game: &Game, choices: &Option<&Choices>, player_index: Option<ids::PlayerIndex>,
		perspective: &Perspective,
	) -> DeckPerspective {
		let visibility = self.spec.visibility.get(perspective);
		match visibility {
			Visibility::Visible => DeckPerspective {
				count: self.num_top_cards(),
				label: self.spec.get_label(),
				stacks: Some(
					self
						.stacks()
						.map(|s| s.to_perspective(game, choices, player_index, self.spec.path))
						.collect(),
				),
				choice_associations: ChoiceAssociation::create_from_choices(
					choices,
					DisplayPath::DeckAt(self.spec.path),
				),
			},
			Visibility::Summary => DeckPerspective {
				count: self.num_top_cards(),
				label: self.spec.get_label(),
				stacks: None,
				choice_associations: ChoiceAssociation::create_from_choices(
					choices,
					DisplayPath::DeckAt(self.spec.path),
				),
			},
			Visibility::Invisible => {
				unreachable!();
			}
		}
	}

	pub fn to_spectator_perspective(
		&self, game: &Game, choices: &Option<&Choices>, player_index: Option<ids::PlayerIndex>,
	) -> DeckPerspective {
		self.to_perspective(game, choices, player_index, &Perspective::Spectator)
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

	pub fn stacks(&self) -> impl Iterator<Item = &Stack> {
		self.stacks.iter()
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

	pub fn hero_types(&self) -> HashSet<HeroType> {
		return self
			.stacks
			.iter()
			.filter_map(|s| s.get_hero_type())
			.collect();
	}

	pub fn take(&mut self, card_id: ids::CardId) -> Option<Stack> {
		// Should be able to take modifiers!!
		self
			.stacks
			.iter()
			.position(|s| s.contains(card_id))
			.and_then(|i| self.stacks.remove(i))
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
					return Some(&modifier);
				}
			}
		}
		None
	}
}

impl Summarizable for Deck {
	fn summarize<W: Write>(
		&self, f: &mut BufWriter<W>, indentation_level: u32,
	) -> Result<(), std::io::Error> {
		for _ in 0..indentation_level {
			write!(f, "  ")?;
		}
		write!(f, "{}: ", self.spec.get_label())?;
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

#[derive(Debug, PartialEq, Clone)]
pub struct DeckPerspective {
	pub label: String,
	pub count: usize,
	pub stacks: Option<Vec<StackPerspective>>,

	pub choice_associations: Vec<ChoiceAssociation>,
}
