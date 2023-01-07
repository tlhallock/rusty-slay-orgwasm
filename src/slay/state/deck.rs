// use super::ids::{CardId, ChallengeId, ChoiceId, DeckId, ElementId, IdGenerator, PlayerId, RollId};

use crate::slay::choices::ChoiceAssociation;
use crate::slay::choices::ChoicesPerspective;
use crate::slay::errors;
use crate::slay::errors::SlayResult;
use crate::slay::ids;
use crate::slay::specification;
use crate::slay::specification::CardType;
use crate::slay::specification::HeroType;
use crate::slay::state::player::Player;
use crate::slay::state::stack::Card;
use crate::slay::state::stack::Stack;
use crate::slay::state::stack::StackPerspective;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::visibility::Perspective;
use crate::slay::visibility::Visibility;

use std::io::Write;

use std::collections::HashSet;
use std::collections::VecDeque;

use std::fmt::Debug;

use std::io::BufWriter;
use std::ops::RangeBounds;

use std::iter::Iterator;

// Lol, tried of looking for the deck by id...
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
	pub id: ids::DeckId,
	stacks: VecDeque<Stack>,
	pub spec: specification::DeckSpec,
}

impl Deck {
	pub fn new(id: ids::DeckId, spec: specification::DeckSpec) -> Self {
		Self {
			id,
			stacks: Default::default(),
			spec,
		}
	}

	pub fn to_perspective(
		&self, perspective: &Perspective, player: Option<&Player>, choices: &Option<ChoicesPerspective>,
	) -> DeckPerspective {
		let visibility = self.spec.visibility.get(perspective);
		match visibility {
			Visibility::Visible => DeckPerspective {
				id: self.id,
				count: self.num_top_cards(),
				label: self.spec.label.to_owned(),
				stacks: Some(
					self
						.iter()
						.map(|s| s.to_perspective(player, choices))
						.collect(),
				),
				choice_associations: ChoiceAssociation::create_from_choices(choices, self.id),
			},
			Visibility::Summary => DeckPerspective {
				id: self.id,
				count: self.num_top_cards(),
				label: self.spec.label.to_owned(),
				stacks: None,
				choice_associations: ChoiceAssociation::create_from_choices(choices, self.id),
			},
			Visibility::Invisible => {
				unreachable!();
			}
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

	pub fn is_visible(&self, perspective: &Perspective) -> bool {
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

	pub fn take_at_index(&mut self, index: usize) -> Stack {
		self.stacks.remove(index).unwrap()
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
		writeln!(f)?;
		Ok(())
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeckPerspective {
	pub id: ids::DeckId,
	pub label: String,
	pub count: usize,
	pub stacks: Option<Vec<StackPerspective>>,

	pub choice_associations: Vec<ChoiceAssociation>,
}
