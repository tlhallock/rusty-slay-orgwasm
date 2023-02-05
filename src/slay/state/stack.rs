use crate::slay::choices::ChoiceDisplayType;
use crate::slay::ids;
use crate::slay::specification::CardSpec;
use crate::slay::specification::HeroType;
use crate::slay::specification::MonsterSpec;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::specs::hero::HeroAbility;
use crate::slay::specs::hero::HeroAbilityType;
use crate::slay::specs::items::AnotherItemType;
use crate::slay::state::game::Game;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::status_effects::effect::HeroStatusEffect;
use crate::slay::status_effects::effect_entry::HeroStatusEffectEntry;

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

	pub fn modification_amounts(&self) -> Vec<i32> {
		self.get_spec().modifiers.to_vec() // ::<Vec<(ids::CardId, i32)>>()
	}

	pub fn label(&self) -> &'static str {
		self.card_type.label()
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
		matches!(self.card_type, SlayCardSpec::HeroCard(_))
	}

	pub(crate) fn is_modifier(&self) -> bool {
		matches!(self.card_type, SlayCardSpec::ModifierCard(_))
	}

	pub(crate) fn is_item(&self) -> bool {
		matches!(self.card_type, SlayCardSpec::Item(_))
	}

	pub(crate) fn is_magic(&self) -> bool {
		matches!(self.card_type, SlayCardSpec::MagicCard(_))
	}

	pub(crate) fn is_challenge(&self) -> bool {
		self.get_spec().is_challenge()
	}

	pub(crate) fn get_unmodified_hero_type(&self) -> Option<HeroType> {
		self.get_spec().get_unmodified_hero_type()
	}
}

pub struct ActiveHeroItem {
	pub hero: HeroAbilityType,
	pub item_id: ids::CardId,
	pub effect: HeroStatusEffectEntry,
}

impl Stack {
	pub fn new(top: Card) -> Self {
		Self {
			top,
			modifiers: Vec::new(),
		}
	}

	pub fn hero_effects(&self) -> impl Iterator<Item = ActiveHeroItem> + '_ {
		// guard_unwrap!(
		// 	let SlayCardSpec::HeroCard(top_hero) = self.top.card_type
		// );

		self.modifiers.iter().map(|item_card| {
			guard_unwrap!(
				let SlayCardSpec::Item(item) = item_card.card_type
			);
			guard_unwrap!(
				let SlayCardSpec::HeroCard(hero_card) = self.top.card_type
			);
			ActiveHeroItem {
				hero: hero_card,
				item_id: item_card.id,
				effect: item.hero_effect_entry(),
			}
		})
	}

	// pub(crate) fn card_has_modifier(&self, modifier: HeroStatusEffect) -> bool {
	// 	let visitor = CardHasModifier::new(modifier);
	// 	// self.tour_buffs(&mut visitor);
	// 	visitor.has
	// }

	pub fn contains(&self, card_id: ids::CardId) -> bool {
		self.top.id == card_id || self.modifiers.iter().any(|c| c.id == card_id)
	}

	pub(crate) fn get_hero_type(&self) -> Option<HeroType> {
		guard!(
			let SlayCardSpec::HeroCard(hero_card) = self.top.card_type
			else { return None; }
		);
		let mut return_value = hero_card.hero_type();
		for item_card in self.modifiers.iter() {
			guard_unwrap!(
				let SlayCardSpec::Item(item) = item_card.card_type
			);
			if let AnotherItemType::MaskCard(hero_type) = item {
				return_value = hero_type;
			}
		}
		Some(return_value)
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
			self.label(),
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
	pub fn get_id_to_sacrifice_or_destroy(&self) -> ids::CardId {
		self
			.hero_effects()
			.find(|item| item.effect.effect == HeroStatusEffect::SacrificeMeInstead)
			.map(|item| item.item_id)
			.unwrap_or(self.top.id)
	}
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
