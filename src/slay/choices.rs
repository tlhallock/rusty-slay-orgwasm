use crate::common::perspective::RollModificationChoice;
use crate::slay::errors::SlayResult;
use crate::slay::game_context;
use crate::slay::ids;
use crate::slay::state;

use std::fmt::Debug;

use super::deadlines::Timeline;
use super::game_context::GameBookKeeping;
use super::showdown::common::ModificationPath;
use super::state::Game;
use super::tasks::PlayerTask;

#[derive(Clone, Debug)]
pub struct Choices {
	pub instructions: String,
	pub options: Vec<TasksChoice>,
	pub default_choice: ids::ChoiceId,
	pub timeline: Timeline,
}

impl Choices {
	pub fn new(
		options: Vec<TasksChoice>, default_choice: ids::ChoiceId, timeline: Timeline,
		instructions: String,
	) -> Self {
		Self {
			options,
			default_choice,
			timeline,
			instructions,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayPath {
	DeckAt(state::DeckPath),
	CardIn(state::DeckPath, ids::CardId),
	Player(usize),
	Roll(ModificationPath),
}

#[derive(Debug, Clone)]
pub struct DisplayArrow {
	pub source: DisplayPath,
	pub destination: DisplayPath,
}

#[derive(Debug, Clone, Default)]
pub struct ChoiceDisplay {
	pub highlight: Option<DisplayPath>,
	pub arrows: Vec<DisplayArrow>,
	// pub references_id: Option<ids::ElementId>,

	// Replace the following string with some useful enum...
	pub label: String,

	pub roll_modification_choice: Option<RollModificationChoice>,
}

#[derive(Debug, Clone)]
pub struct ChoiceLocator {
	pub id: ids::ChoiceId,
	pub player_index: usize,
}

#[derive(Debug, Clone)]
pub struct ChoiceInformation {
	pub locator: ChoiceLocator,
	pub display: ChoiceDisplay,
}

impl ChoiceInformation {
	pub fn new(locator: ChoiceLocator, display: ChoiceDisplay) -> Self {
		Self { locator, display }
	}

	pub fn get_id(&self) -> ids::ChoiceId {
		self.locator.id
	}
	pub fn player_index(&self) -> usize {
		self.locator.player_index
	}
}

dyn_clone::clone_trait_object!(Choice);

pub trait Choice: Debug + dyn_clone::DynClone {
	fn select(
		&mut self, context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<()>;

	fn get_choice_information(&self) -> &ChoiceInformation;
}

// impl<'de> Deserialize<'de> for Box<dyn Choice> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de> {
//         todo!()
//     }
// }

#[derive(Debug, Clone)]
pub struct TasksChoice {
	choice_information: ChoiceInformation,
	tasks: Vec<Box<dyn PlayerTask>>,
	prepend: bool,
}

impl TasksChoice {
	pub fn new(choice_information: ChoiceInformation, tasks: Vec<Box<dyn PlayerTask>>) -> Self {
		Self {
			choice_information,
			tasks,
			prepend: false,
		}
	}
	pub fn prepend(choice_information: ChoiceInformation, tasks: Vec<Box<dyn PlayerTask>>) -> Self {
		Self {
			choice_information,
			tasks,
			prepend: true,
		}
	}
}

impl Choice for TasksChoice {
	fn select(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game,
	) -> super::errors::SlayResult<()> {
		let player_index = self.choice_information.player_index();
		if self.prepend {
			game.players[player_index]
				.tasks
				.prepend_from(&mut self.tasks);
		} else {
			game.players[player_index].tasks.take_from(&mut self.tasks);
		}
		Ok(())
	}

	fn get_choice_information(&self) -> &ChoiceInformation {
		&self.choice_information
	}
}
