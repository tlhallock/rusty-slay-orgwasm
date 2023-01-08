use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;

use crate::slay::deadlines::Timeline;
use crate::slay::ids;
use crate::slay::showdown::common::ModificationPath;
use crate::slay::showdown::common::RollModificationChoiceType;
use crate::slay::showdown::completion::Completion;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::CardSpecPerspective;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::tasks::MoveCardTask;
use crate::slay::tasks::PlayerTask;

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

	pub fn choice_perspetives(&self) -> Vec<ChoicePerspective> {
		self
			.options
			.iter()
			.map(|choice| choice.to_perspective(self.default_choice.iter().any(|dc| *dc == choice.id)))
			.collect()
	}

	pub fn to_perspective(&self) -> ChoicesPerspective {
		ChoicesPerspective {
			timeline: self.timeline.to_owned(),
			instructions: self.instructions.to_owned(),
			actions: self.choice_perspetives(),
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
			write!(f, "'{}', ", option.display.label)?;
		}
		writeln!(f)?;
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum CardPath {
	TopCardIn(DeckPath, ids::CardId),
	ModifyingCardIn(DeckPath, ids::CardId, ids::CardId),
	Leader(ids::PlayerIndex),
}

impl CardPath {
	pub fn display(&self) -> DisplayPath {
		DisplayPath::CardAt(*self)
	}

	pub fn get_deck_path(&self) -> DeckPath {
		match self {
			CardPath::TopCardIn(dp, _) => *dp,
			CardPath::ModifyingCardIn(dp, _, _) => *dp,
			CardPath::Leader(_) => unreachable!(),
		}
	}

	pub fn get_card_id(&self) -> ids::CardId {
		match self {
			CardPath::TopCardIn(_, card_id) => *card_id,
			CardPath::ModifyingCardIn(_, _, card_id) => *card_id,
			CardPath::Leader(_card_id) => todo!(),
		}
	}

	pub fn get_place_task(&self) -> Box<dyn PlayerTask> {
		self.get_move_task(DeckPath::Party(self.get_player_index().unwrap()))
	}

	pub fn get_player_index(&self) -> Option<ids::PlayerIndex> {
		self.get_deck_path().get_player_index()
	}

	pub fn get_move_task(&self, destination: DeckPath) -> Box<dyn PlayerTask> {
		match self {
			CardPath::TopCardIn(deck_path, card_id) => Box::new(MoveCardTask {
				source: *deck_path,
				destination,
				card_id: *card_id,
			}),
			CardPath::ModifyingCardIn(deck_path, _, card_id) => Box::new(MoveCardTask {
				source: *deck_path,
				destination,
				card_id: *card_id,
			}),
			CardPath::Leader(_) => unreachable!(),
		}
	}

	pub fn get_discard_task(&self) -> Box<dyn PlayerTask> {
		self.get_move_task(DeckPath::Discard)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum DisplayPath {
	DeckAt(DeckPath),
	CardAt(CardPath),
	Player(ids::PlayerIndex),
	Roll(ModificationPath),
}

impl DisplayPath {
	pub fn to_highlight(&self) -> ChoiceDisplayType {
		ChoiceDisplayType::HighlightPath(*self)
	}
}

#[derive(Debug, Clone)]
pub struct DisplayArrow {
	pub source: DisplayPath,
	pub destination: DisplayPath,
}

#[derive(Debug, Clone)]
pub struct ChoiceDisplay {
	// pub arrows: Vec<DisplayArrow>,
	pub display_type: ChoiceDisplayType,
	pub label: String,
	// pub highlight: Option<DisplayPath>,
	// pub references_id: Option<ids::ElementId>,

	// TODO: Replace the following string with some useful enum...
	// pub label: String,
	// TODO: get rid of this...
	// pub roll_modification_choice: Option<RollModificationChoice>,
}

// impl Default for ChoiceDisplay {
//     fn default() -> Self {
//         Self {
// 					highlight: Default::default(),
// 					arrows: Default::default(),
// 					display_type: ChoiceDisplayType::Text("Please fill in the text for this choice"),
// 					label: Default::default(),
// 					roll_modification_choice: Default::default()
// 				}
//     }
// }

// #[derive(Debug, Clone)]
// pub struct ChoiceLocator {
// 	pub id: ids::ChoiceId,
// 	pub player_index: ids::PlayerIndex,
// }

// #[derive(Debug, Clone)]
// pub struct ChoiceInformation {
// 	pub locator: ChoiceLocator,
// 	pub display: ChoiceDisplay,
// }

// impl ChoiceInformation {
// 	pub fn new(locator: ChoiceLocator, display: ChoiceDisplay) -> Self {
// 		Self { locator, display }
// 	}

// 	pub fn get_id(&self) -> ids::ChoiceId {
// 		self.locator.id
// 	}
// 	pub fn player_index(&self) -> usize {
// 		self.locator.player_index
// 	}
// }

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

// Rename this to Choice
#[derive(Debug, Clone)]
pub struct TasksChoice {
	pub id: ids::ChoiceId,
	pub display: ChoiceDisplay,
	tasks: Vec<Box<dyn PlayerTask>>,
	prepend: bool,
}

impl TasksChoice {
	pub fn new(id: ids::ChoiceId, display: ChoiceDisplay, tasks: Vec<Box<dyn PlayerTask>>) -> Self {
		Self {
			id,
			display,
			tasks,
			prepend: false,
		}
	}
	pub fn prepend(
		id: ids::ChoiceId, display: ChoiceDisplay, tasks: Vec<Box<dyn PlayerTask>>,
	) -> Self {
		Self {
			id,
			display,
			tasks,
			prepend: true,
		}
	}

	pub fn select(
		&mut self, game: &mut Game, player_index: ids::PlayerIndex,
	) -> super::errors::SlayResult<()> {
		if self.prepend {
			game.players[player_index]
				.tasks
				.prepend_from(&mut self.tasks);
		} else {
			game.players[player_index].tasks.take_from(&mut self.tasks);
		}
		Ok(())
	}

	pub fn to_perspective(&self, is_default: bool) -> ChoicePerspective {
		ChoicePerspective {
			is_default,
			choice_id: self.id,
			display_type: self.display.display_type.to_owned(),
			label: self.display.label.to_owned(),
		}
	}
}

// impl Choice for TasksChoice {
// }

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
	fn new(choices: &Choices, choice: &TasksChoice, association_type: ChoiceAssociationType) -> Self {
		Self {
			choice_id: choice.id,
			association_type,
			label: choice.display.label.to_owned(),
			is_default: choices.default_choice.iter().any(|id| *id == choice.id),
		}
	}

	fn create_from_choice(choices: &Choices, choice: &TasksChoice, path: DisplayPath) -> Vec<Self> {
		let mut ret = Vec::new();
		if let ChoiceDisplayType::HighlightPath(display_path) = choice.display.display_type {
			if path == display_path {
				ret.push(ChoiceAssociation::new(
					choices,
					choice,
					ChoiceAssociationType::Representer,
				));
			}
		}
		//  lol
		// // Not even sure if this will be used...
		// let (already_source, already_destination) = (false, false);
		// for arrow in choice.display.arrows.iter() {
		// 	if arrow.source == path && !already_source {
		// 			ret.push(ChoiceAssociation::new(
		// 					choices, choice, ChoiceAssociationType::Source));
		// 	}
		// 	if arrow.destination == path && !already_destination {
		// 			ret.push(ChoiceAssociation::new(
		// 					choices, choice, ChoiceAssociationType::Destination));
		// 	}
		// }
		ret
	}

	pub fn create_from_choices(choices: &Option<&Choices>, path: DisplayPath) -> Vec<Self> {
		if let Some(choices) = choices {
			choices
				.options
				.iter()
				.flat_map(|choice| ChoiceAssociation::create_from_choice(choices, choice, path))
				.collect()
		} else {
			Vec::new()
		}
	}
}

// Defines how this choice should be viewed.
#[derive(Debug, PartialEq, Clone)]
pub enum ChoiceDisplayType {
	HighlightPath(DisplayPath),
	Modify(RollModificationChoiceType),
	Challenge(CardSpecPerspective),
	SetCompletion(Completion),
	Text(&'static str),
	Card(CardSpecPerspective),
	Yes,
	No,
	Forfeit,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChoicePerspective {
	pub is_default: bool,
	pub choice_id: ids::ChoiceId,
	pub display_type: ChoiceDisplayType,
	pub label: String,
	// pub roll_modification_choice: Option<RollModificationChoice>,
	// Should we add another one of these for card actions? ^^
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChoicesPerspective {
	pub instructions: String,
	pub timeline: Timeline,
	pub actions: Vec<ChoicePerspective>,
}
