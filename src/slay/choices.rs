use crate::common::perspective::RollModificationChoice;
use crate::slay::errors::SlayResult;
use crate::slay::game_context;
use crate::slay::ids;
use crate::slay::state;

use std::fmt::Debug;

use super::deadlines::Timeline;
use super::showdown::common::ModificationPath;

#[derive(Clone, Debug)]
pub struct Choices {
	pub instructions: String,
	pub options: Vec<Box<dyn Choice>>,
	pub default_choice: ids::ChoiceId,
	pub timeline: Timeline,
}

impl Choices {
	pub fn new(
		options: Vec<Box<dyn Choice>>, default_choice: ids::ChoiceId, timeline: Timeline,
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

#[derive(Debug, Clone)]
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
