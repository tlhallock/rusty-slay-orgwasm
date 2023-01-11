use crate::slay::choices::ChoiceDisplayType;
use crate::slay::ids;
use crate::slay::modifiers::ItemModifier;
use crate::slay::specification::CardSpec;
use crate::slay::specification::HeroType;
use crate::slay::specification::MonsterSpec;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::specs::hero::HeroAbility;
use crate::slay::state::game::Game;
use crate::slay::state::summarizable::Summarizable;

use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;
use std::iter::Iterator;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
	pub id: ids::CardId,
	pub card_type: SlayCardSpec,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CardPerspective {
	pub id: ids::CardId,
	pub spec: SlayCardSpec,
	pub played_this_turn: bool,
}

#[derive(Debug, Clone)]
pub struct Stack {
	// pub id: ElementId,
	pub top: Card,
	pub modifiers: Vec<Card>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StackPerspective {
	pub top: CardPerspective,
	pub modifiers: Vec<CardPerspective>,
}

impl Card {
	pub fn get_spec(&self) -> CardSpec {
		self.card_type.get_card_spec_creation()
	}

	pub fn new(id: ids::CardId, card_type: SlayCardSpec) -> Self {
		Card { id, card_type }
	}

	pub fn to_perspective(&self, played_this_turn: bool) -> CardPerspective {
		CardPerspective {
			id: self.id,
			played_this_turn,
			spec: self.card_type,
		}
	}

	pub(crate) fn is_magic(&self) -> bool {
		self.get_spec().is_magic()
	}

	pub fn modification_amounts(&self) -> Vec<i32> {
		self.get_spec().modifiers.to_vec() // ::<Vec<(ids::CardId, i32)>>()
	}

	pub fn label(&self) -> String {
		self.get_spec().label
	}

	pub fn monster_spec(&self) -> Option<MonsterSpec> {
		self
			.get_spec()
			.monster
			.as_ref()
			.map(|monster| monster.create_spec())
	}

	pub fn hero_ability(&self) -> Option<HeroAbility> {
		self.get_spec().hero_ability
	}

	pub fn as_choice(&self) -> ChoiceDisplayType {
		ChoiceDisplayType::Card_(self.card_type)
	}

	pub(crate) fn is_hero(&self) -> bool {
		self.get_spec().is_hero()
	}

	pub(crate) fn is_challenge(&self) -> bool {
		self.get_spec().is_challenge()
	}

	pub(crate) fn get_unmodified_hero_type(&self) -> Option<HeroType> {
		self.get_spec().get_unmodified_hero_type()
	}
}

impl Stack {
	pub fn new(top: Card) -> Self {
		Self {
			top,
			modifiers: Vec::new(),
		}
	}

	pub fn contains(&self, card_id: ids::CardId) -> bool {
		self.top.id == card_id || self.modifiers.iter().any(|c| c.id == card_id)
	}

	pub(crate) fn get_hero_type(&self) -> Option<HeroType> {
		if let Some(hero_type) = self.top.get_spec().get_unmodified_hero_type() {
			let mut ret = hero_type;
			for modifier in self.modifiers.iter() {
				if let Some(ItemModifier::Mask(hero_type)) = modifier.get_spec().card_modifier.as_ref() {
					ret = *hero_type;
				}
			}
			Some(ret)
		} else {
			None
		}
	}
}

impl Summarizable for Card {
	fn summarize<W: Write>(
		&self,
		f: &mut BufWriter<W>,
		_indentation_level: u32,
	) -> Result<(), std::io::Error> {
		write!(
			f,
			"({}) {}",
			self.id,
			self.get_spec().label,
			//  if self.played_this_turn { "X" } else { "" }
		)
	}
}
impl Summarizable for Stack {
	fn summarize<W: Write>(
		&self,
		f: &mut BufWriter<W>,
		indentation_level: u32,
	) -> Result<(), std::io::Error> {
		write!(f, "[")?;
		self.top.summarize(f, indentation_level + 1)?;
		write!(f, " {}], ", self.modifiers.len())
	}
}

impl Stack {
	pub fn to_perspective(
		&self,
		game: &Game,
		player_index: Option<ids::PlayerIndex>,
	) -> StackPerspective {
		StackPerspective {
			top: self.top.to_perspective(
				game.was_card_played(player_index, self.top.id), // DisplayPath::CardAt(CardPath::TopCardIn(deck_path, self.top.id)),
			),
			modifiers: self
				.modifiers
				.iter()
				.map(|s| s.to_perspective(false))
				.collect(),
		}
	}
}

// // Remove this class...
// #[derive(Debug, PartialEq, Clone)]
// pub struct CardSpecPerspective {
// 	// pub card_type: CardType,
// 	pub label: String,
// 	pub description: String,
// 	pub image_path: String,
// 	// pub monster: Option<MonsterSpec>,
// 	pub modifiers: Vec<i32>,
// }

// impl CardSpecPerspective {
// 	pub fn new(spec: &CardSpec) -> Self {
// 		Self {
// 			// card_type: spec.card_type.to_owned(),
// 			label: spec.label.to_owned(),
// 			description: spec.description.to_owned(),
// 			image_path: spec.image_path.to_owned(),
// 			// monster: spec.monster.to_owned(),
// 			modifiers: spec.modifiers.to_owned(),
// 		}
// 	}
// }
