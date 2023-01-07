use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;

use crate::slay::deadlines::Timeline;

use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::showdown::common::ModificationPath;

use crate::slay::state::game::Game;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::tasks::PlayerTask;

use super::showdown::common::RollModificationChoice;
use super::state::deck::DeckPath;

#[derive(Clone, Debug)]
pub struct Choices {
	pub instructions: String,
	pub options: Vec<TasksChoice>,
	pub default_choice: Option<ids::ChoiceId>,
	pub timeline: Timeline,
}

impl Choices {
	pub fn new(
		options: Vec<TasksChoice>, default_choice: Option<ids::ChoiceId>, timeline: Timeline,
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

impl Summarizable for Choices {
	fn summarize<W: Write>(
		&self, f: &mut BufWriter<W>, indentation_level: u32,
	) -> Result<(), std::io::Error> {
		for _ in 0..indentation_level {
			write!(f, "  ")?;
		}
		write!(f, "choices: ({}): ", self.instructions)?;
		for option in self.options.iter() {
			write!(f, "'{}', ", option.get_choice_information().display.label)?;
		}
		writeln!(f)?;
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayPath {
	DeckAt(DeckPath),
	CardIn(DeckPath, ids::CardId),
	Player(ids::PlayerIndex),
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
	pub player_index: ids::PlayerIndex,
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

// dyn_clone::clone_trait_object!(Choice);

// pub trait Choice: Debug + dyn_clone::DynClone {
// 	fn select(
// 		&mut self, context: &mut GameBookKeeping, game: &mut Game,
// 	) -> SlayResult<()>;

// 	fn get_choice_information(&self) -> &ChoiceInformation;
// }

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
	pub fn select(
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

	pub fn get_choice_information(&self) -> &ChoiceInformation {
		&self.choice_information
	}
}

// impl Choice for TasksChoice {
// }

impl Choices {
	pub fn to_perspective(&self, game: &Game) -> ChoicesPerspective {
		ChoicesPerspective {
			timeline: self.timeline.to_owned(),
			instructions: self.instructions.to_owned(),
			actions: self
				.options
				.iter()
				.map(|o| {
					let info = o.get_choice_information();
					ChoicePerspective {
						is_default: self.default_choice.iter().any(|dc| *dc == info.get_id()),
						choice_id: info.get_id(),
						label: info.display.label.to_owned(),
						highlight: game.get_element_id(&info.display.highlight),
						highlight_path: info.display.highlight.to_owned(),
						arrows: vec![],
						roll_modification_choice: info.display.roll_modification_choice.to_owned(),
					}
				})
				.collect(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum ChoiceAssociationType {
	Representer,
	Source,
	Destination,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChoiceAssociation {
	pub choice_id: ids::ChoiceId,
	pub association_type: ChoiceAssociationType,
	pub label: String,
	pub is_default: bool,
}

impl ChoiceAssociation {
	fn new(association_type: ChoiceAssociationType, choice_info: &ChoicePerspective) -> Self {
		Self {
			choice_id: choice_info.choice_id,
			association_type,
			label: choice_info.label.to_owned(),
			is_default: choice_info.is_default,
		}
	}

	fn create_from_choice(choice_info: &ChoicePerspective, id: ids::ElementId) -> Vec<Self> {
		let mut ret = Vec::new();
		ret.extend(
			choice_info
				.highlight
				.iter()
				.filter(|highlight_id| **highlight_id == id)
				.map(|_| ChoiceAssociation::new(ChoiceAssociationType::Representer, choice_info)),
		);
		ret.extend(choice_info.arrows.iter().flat_map(|a| {
			let mut ret: Vec<ChoiceAssociation> = Vec::new();
			if let Some(source_id) = a.0 {
				if source_id == id {
					ret.push(ChoiceAssociation::new(
						ChoiceAssociationType::Source,
						choice_info,
					))
				}
			}
			if let Some(source_id) = a.1 {
				if source_id == id {
					ret.push(ChoiceAssociation::new(
						ChoiceAssociationType::Destination,
						choice_info,
					))
				}
			}
			ret
		}));
		ret
	}

	pub fn create_from_choices(
		choices: &Option<ChoicesPerspective>, id: ids::ElementId,
	) -> Vec<Self> {
		if let Some(choices2) = choices {
			return choices2
				.actions
				.iter()
				.flat_map(|a| ChoiceAssociation::create_from_choice(a, id))
				.collect();
		}
		vec![]
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChoicePerspective {
	pub is_default: bool,
	pub choice_id: ids::ChoiceId,
	pub label: String,
	// Which one is better...
	pub highlight: Option<ids::ElementId>,
	pub highlight_path: Option<DisplayPath>,
	// This will probably need an arrow id...
	pub arrows: Vec<(Option<ids::ElementId>, Option<ids::ElementId>)>,

	pub roll_modification_choice: Option<RollModificationChoice>,
	// Should we add another one of these for card actions? ^^
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChoicesPerspective {
	pub instructions: String,
	pub timeline: Timeline,
	pub actions: Vec<ChoicePerspective>,
}
