use crate::slay::choices::CardPath;
use crate::slay::choices::ChoiceAssociation;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayPath;
use crate::slay::ids;
use crate::slay::modifiers::ItemModifier;
use crate::slay::specification;
use crate::slay::specification::CardSpec;
use crate::slay::specification::CardType;
use crate::slay::specification::HeroAbility;
use crate::slay::specification::HeroType;
use crate::slay::specification::MonsterSpec;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::summarizable::Summarizable;

use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;
use std::iter::Iterator;

#[derive(Debug, Clone)]
pub struct Card {
	pub id: ids::CardId,
	pub spec: CardSpec,
}

impl Card {
	pub fn new(id: ids::CardId, spec: CardSpec) -> Self {
		Card { id, spec }
	}

	pub(crate) fn is_magic(&self) -> bool {
		self.spec.is_magic()
	}

	pub fn modification_amounts(&self) -> Vec<i32> {
		self.spec.modifiers.to_vec() // ::<Vec<(ids::CardId, i32)>>()
	}

	pub fn label(&self) -> String {
		self.spec.label.to_string()
	}

	pub fn monster_spec(&self) -> Option<MonsterSpec> {
		if let Some(monster) = &self.spec.monster {
			Some(monster.create_spec())
		} else {
			None
		}
	}

	pub fn hero_ability(&self) -> &Option<HeroAbility> {
		&self.spec.hero_ability
	}

	// Should be part of the spec...
	pub fn as_perspective(&self) -> CardSpecPerspective {
		CardSpecPerspective::new(&self.spec)
	}

	pub fn as_choice(&self) -> ChoiceDisplayType {
		ChoiceDisplayType::Card(self.as_perspective())
	}

	pub(crate) fn is_hero(&self) -> bool {
		self.spec.is_hero()
	}

	pub(crate) fn is_challenge(&self) -> bool {
		self.spec.is_challenge()
	}

	pub(crate) fn get_unmodified_hero_type(&self) -> Option<HeroType> {
		self.spec.get_unmodified_hero_type()
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

	pub fn contains(&self, card_id: ids::CardId) -> bool {
		self.top.id == card_id || self.modifiers.iter().any(|c| c.id == card_id)
	}

	pub(crate) fn get_hero_type(&self) -> Option<HeroType> {
		if let Some(hero_type) = self.top.spec.get_unmodified_hero_type() {
			let mut ret = hero_type;
			for modifier in self.modifiers.iter() {
				if let Some(ItemModifier::Mask(hero_type)) = modifier.spec.card_modifier.as_ref() {
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
		&self, f: &mut BufWriter<W>, _indentation_level: u32,
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

impl Card {
	pub fn to_perspective(
		&self, game: &Game, choices: &Option<&Choices>, player_index: Option<ids::PlayerIndex>,
		card_path: DisplayPath,
	) -> CardPerspective {
		CardPerspective {
			id: self.id,
			played_this_turn: game.was_card_played(player_index, self.id),
			spec: CardSpecPerspective::new(&self.spec),
			choice_associations: ChoiceAssociation::create_from_choices(choices, card_path),
		}
	}
}

impl Stack {
	pub fn to_perspective(
		&self, game: &Game, choices: &Option<&Choices>, player_index: Option<ids::PlayerIndex>,
		deck_path: DeckPath,
	) -> StackPerspective {
		StackPerspective {
			top: self.top.to_perspective(
				game,
				choices,
				player_index,
				DisplayPath::CardAt(CardPath::TopCardIn(deck_path, self.top.id)),
			),
			modifiers: self
				.modifiers
				.iter()
				.map(|s| {
					s.to_perspective(
						game,
						choices,
						player_index,
						DisplayPath::CardAt(CardPath::ModifyingCardIn(deck_path, self.top.id, s.id)),
					)
				})
				.collect(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct CardSpecPerspective {
	// pub card_type: CardType,
	pub label: String,
	pub description: String,
	pub image_path: String,
	// pub monster: Option<MonsterSpec>,
	pub modifiers: Vec<i32>,
}

impl CardSpecPerspective {
	pub fn new(spec: &CardSpec) -> Self {
		Self {
			// card_type: spec.card_type.to_owned(),
			label: spec.label.to_owned(),
			description: spec.description.to_owned(),
			image_path: spec.image_path.to_owned(),
			// monster: spec.monster.to_owned(),
			modifiers: spec.modifiers.to_owned(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct CardPerspective {
	pub id: ids::CardId,
	pub played_this_turn: bool,
	pub spec: CardSpecPerspective,
	pub choice_associations: Vec<ChoiceAssociation>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StackPerspective {
	pub top: CardPerspective,
	pub modifiers: Vec<CardPerspective>,
}
