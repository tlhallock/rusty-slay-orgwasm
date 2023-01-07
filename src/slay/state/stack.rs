
use crate::slay::choices::ChoiceAssociation;
use crate::slay::choices::DisplayPath;
// use super::ids::{CardId, ChallengeId, ChoiceId, DeckId, ElementId, IdGenerator, PlayerId, RollId};
use crate::slay::ids;
use crate::slay::modifiers::PlayerBuffs;
use crate::slay::state::player::Player;
use crate::slay::specification;
use crate::slay::specification::CardSpec;
use crate::slay::specification::CardType;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::PlayerTasks;
use crate::slay::choices;
use crate::slay::choices::ChoicesPerspective;
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

use std::fmt::Debug;

use std::io::BufWriter;
use std::ops::RangeBounds;

use std::iter::Iterator;

use super::summarizable::Summarizable;

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
		&self, player: Option<&Player>, choices: &Option<ChoicesPerspective>,
	) -> CardPerspective {
		CardPerspective {
			id: self.id,
			played_this_turn: player.iter().any(|p| p.was_card_played(&self.id)),
			spec: CardSpecPerspective::new(&self.spec),
			choice_associations: ChoiceAssociation::create_from_choices(choices, self.id),
		}
	}
}



impl Stack {
	pub fn to_perspective(
		&self, player: Option<&Player>, choices: &Option<ChoicesPerspective>,
	) -> StackPerspective {
		StackPerspective {
			top: self.top.to_perspective(player, choices),
			modifiers: self
				.modifiers
				.iter()
				.map(|s| s.to_perspective(player, choices))
				.collect(),
		}
	}
}


#[derive(Debug, PartialEq, Clone)]
pub struct CardSpecPerspective {
	pub card_type: CardType,
	pub label: String,
	pub description: String,
	pub image_path: String,
	// pub monster: Option<MonsterSpec>,
	pub modifiers: Vec<i32>,
}

impl CardSpecPerspective {
	pub fn new(spec: &CardSpec) -> Self {
		Self {
			card_type: spec.card_type.to_owned(),
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