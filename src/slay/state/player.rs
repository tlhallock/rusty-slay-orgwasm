use crate::slay::choices::ChoicePerspective;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesPerspective;
use crate::slay::choices::DisplayPath;
use crate::slay::errors;
use crate::slay::ids;
use crate::slay::modifiers::PlayerBuffs;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::showdown::common::RollModification;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::specification::HeroType;
use crate::slay::state::deck::Deck;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::deck::DeckPerspective;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::state::turn::Turn;
use crate::slay::tasks;
use crate::slay::tasks::PlayerTasks;
use crate::slay::visibility::Perspective;
use crate::slay::visibility::VisibilitySpec;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;
use std::iter::Iterator;


use super::deck::DeckSpec;
use super::game::GamePerspective;
use super::game::GameStaticInformation;

pub struct HeroTypeCounter {
	counts: HashMap<HeroType, u32>,
}

impl HeroTypeCounter {
	pub fn new() -> Self {
		Self {
			counts: Default::default(),
		}
	}
	pub fn add_hero_type(&mut self, hero_type: &HeroType) {
		self.counts.insert(
			*hero_type,
			if let Some(prev) = self.counts.get(hero_type) {
				prev + 1
			} else {
				1
			},
		);
	}

	pub(crate) fn maybe_add_hero_type(&mut self, hero_type_option: Option<HeroType>) {
		if let Some(hero_type) = hero_type_option.as_ref() {
			self.add_hero_type(hero_type);
		}
	}
}

#[derive(Clone, Debug)]
pub struct Player {
	pub player_index: ids::PlayerIndex,
	pub name: String,

	pub temporary_buffs: PlayerBuffs,
	pub choices: Option<Choices>,
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
		&mut self,
		task: Box<dyn tasks::PlayerTask>,
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
	pub fn new(name: String, player_index: ids::PlayerIndex, leader: Card) -> Self {
		Player {
			player_index,
			name,
			choices: None,
			tasks: Default::default(),
			remaining_action_points: 0,
			leader,
			temporary_buffs: Default::default(),
			hand: Deck::new(DeckSpec {
				visibility: VisibilitySpec::summary(),
				path: DeckPath::Hand(player_index),
			}),
			party: Deck::new(DeckSpec {
				visibility: VisibilitySpec::visible(),
				path: DeckPath::Party(player_index),
			}),
			slain_monsters: Deck::new(DeckSpec {
				visibility: VisibilitySpec::visible(),
				path: DeckPath::SlainMonsters(player_index),
			}),
			played_this_turn: Default::default(),
		}
	}

	pub fn turn_begin(&mut self) {
		self.remaining_action_points = self.calculate_total_action_points();
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

	pub fn count_hero_types(&self, hero_types: &mut HeroTypeCounter) {
		// Could this be a one liner?
		self.party.count_hero_types(hero_types);
		hero_types.maybe_add_hero_type(self.leader.get_unmodified_hero_type());
	}
	pub fn collect_hero_types(&self, hero_types: &mut HashSet<HeroType>) {
		// Could this be a one liner?
		self.party.collect_hero_types(hero_types);
		hero_types.insert(self.leader.get_unmodified_hero_type().unwrap());
	}
	pub fn has_hero_type(&self, hero_type: &HeroType) -> bool {
		self.leader.get_unmodified_hero_type().unwrap() == *hero_type
			|| self.party.contains_hero_type(hero_type)
	}

	pub fn take_current_task(&mut self) -> Option<Box<dyn tasks::PlayerTask>> {
		self.tasks.take_current_task()
	}

	pub fn clear_expired_modifiers(&mut self, turn: &Turn) {
		self.temporary_buffs.clear_expired_modifiers(turn);
	}

	pub(crate) fn get_remaining_action_points(&self) -> u32 {
		self.remaining_action_points
	}
	pub(crate) fn calculate_total_action_points(&self) -> u32 {
		3 + self
			.slain_monsters
			.tops()
			.map(|card| {
				if let Some(monster) = card.card_type.get_card_spec_creation().monster {
					monster
						.create_spec()
						.modifiers
						.iter()
						.map(|modifier| match modifier {
							PlayerModifier::ExtraActionPoint => 1,
							_ => 0,
						})
						.sum::<u32>()
				} else {
					unreachable!()
				}
			})
			.sum::<u32>()
	}

	pub(crate) fn collect_roll_buffs(&self, _reason: RollReason, ret: &mut Vec<RollModification>) {
		self.temporary_buffs.collect_roll_buffs(ret);

		todo!()
	}

	pub fn to_perspective(
		&self,
		game: &Game,
		perspective: &Perspective,
	) -> PlayerPerspective {
		PlayerPerspective {
			player_index: self.player_index,
			remaining_action_points: self.get_remaining_action_points(),
			// Could be calculated...
			total_action_points: self.calculate_total_action_points(),
			decks: self
				.decks()
				.iter()
				.filter(|d| d.is_visible(perspective))
				.map(|d| d.to_perspective(game, Some(self.player_index), perspective))
				.collect(),
		}
	}
}

impl Summarizable for Player {
	fn summarize<W: Write>(
		&self,
		f: &mut BufWriter<W>,
		indentation_level: u32,
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

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerPerspective {
	pub remaining_action_points: u32,
	pub total_action_points: u32,
	pub decks: Vec<DeckPerspective>,
	player_index: usize,
}

impl PlayerPerspective {
	// List the hero types they have
	// Maybe this -> pub choices: Option<Choices>,

	pub fn name<'a>(&self, statics: &'a GameStaticInformation) -> &'a String {
		&statics.players[self.player_index].name
	}

	pub fn is_me(&self, statics: &GameStaticInformation) -> bool {
		self.player_index == statics.player_index
	}
	pub fn is_active(&self, game: &GamePerspective) -> bool {
		self.player_index == game.turn.active_player_index()
	}
	pub fn choices(&self, choices: &Option<ChoicesPerspective>) -> Vec<ChoicePerspective> {
		if let Some(choices) = choices {
			choices.represented_by(&DisplayPath::Player(self.player_index))
		} else {
			Vec::new()
		}
	}
}
