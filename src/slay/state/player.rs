
use crate::slay::choices::ChoiceAssociation;
use crate::slay::choices::ChoicesPerspective;
use crate::slay::choices::DisplayPath;
use crate::slay::specification::DeckSpec;
use crate::slay::state::deck::Deck;
// use super::ids::{CardId, ChallengeId, ChoiceId, DeckId, ElementId, IdGenerator, PlayerId, RollId};
use crate::slay::ids;
use crate::slay::modifiers::PlayerBuffs;
use crate::slay::specification;
use crate::slay::specification::CardSpec;
use crate::slay::specification::CardType;
use crate::slay::state::stack::Card;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::PlayerTasks;
use crate::slay::choices;
use crate::slay::errors;
use crate::slay::game_context;
use crate::slay::modifiers;
use crate::slay::showdown::current_showdown::CurrentShowdown;
use crate::slay::visibility::Perspective;
use crate::slay::visibility::VisibilitySpec;
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

use super::deck::DeckPerspective;
use super::game::Turn;
use super::stack::CardPerspective;
use super::summarizable::Summarizable;


#[derive(Clone, Debug)]
pub struct Player {
	pub id: ids::PlayerId,
	pub player_index: ids::PlayerIndex,
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
		id_gen: &mut ids::IdGenerator, name: String, player_index: ids::PlayerIndex, leader: Card,
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
			hand: Deck::new(
				id_gen.generate(),
				DeckSpec {
					visibility: VisibilitySpec::summary(),
					label: "Hand".to_string(),
				},
			),
			party: Deck::new(
				id_gen.generate(),
				DeckSpec {
					visibility: VisibilitySpec::visible(),
					label: "Party".to_string(),
				},
			),
			slain_monsters: Deck::new(
				id_gen.generate(),
				specification::DeckSpec {
					visibility: VisibilitySpec::visible(),
					label: "Slain monsters".to_string(),
				},
			),
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

	pub fn clear_expired_modifiers(&mut self, turn: &Turn) {
		self.buffs.clear_expired_modifiers(turn);
	}

	pub(crate) fn get_remaining_action_points(&self) -> u32 {
		self.remaining_action_points
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




#[derive(Debug, PartialEq, Clone)]
pub struct PlayerPerspective {
	// List the hero types they have
	// Are they active player?
	// Is this you?
	// action points
	pub id: ids::PlayerId,
	pub name: String,
	pub me: bool,
	pub active: bool,
	// pub choices: Option<Choices>,
	pub remaining_action_points: u32,
	pub total_action_points: u32,

	pub leader: CardPerspective,
	pub decks: Vec<DeckPerspective>,

	pub choice_associations: Vec<ChoiceAssociation>,
}

impl Player {
	pub fn to_perspective(
		&self, perspective: &Perspective, choices: &Option<ChoicesPerspective>, active: bool,
	) -> PlayerPerspective {
		PlayerPerspective {
			id: self.id,
			name: self.name.to_owned(),
			remaining_action_points: self.get_remaining_action_points(),
			leader: self.leader.to_perspective(Some(self), choices),
			decks: self
				.decks()
				.iter()
				.filter(|d| d.is_visible(perspective))
				.map(|d| d.to_perspective(perspective, Some(self), choices))
				.collect(),
			me: perspective == &Perspective::Owner,
			active,
			choice_associations: ChoiceAssociation::create_from_choices(choices, self.id),
			total_action_points: 3,
		}
	}
}