
use crate::slay::choices::CardPath;
use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::Deck;
use crate::slay::state::deck::PartialDeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::CardSpecPerspective;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskParamName;
use crate::slay::tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct CardChoiceInformation {
	pub card_id: ids::CardId,
	pub display_path: DisplayPath, // This has an unnessesary clone...
	pub card_label: String,        // This has an unnessesary clone...
	pub perspective: CardSpecPerspective,
}

// #[derive(Clone, Debug)]
// pub struct ChooseCardParameterTask {
// 	pub param_name: TaskParamName,
// 	pub instructions: String,
// 	pub card_choices: Vec<CardChoiceInformation>,
// }

// impl PlayerTask for ChooseCardParameterTask {
// 	fn make_progress(
// 		&mut self, context: &mut GameBookKeeping, game: &mut Game, chooser_index: ids::PlayerIndex,
// 	) -> SlayResult<TaskProgressResult> {
// 		unreachable!();
// 		Ok(TaskProgressResult::TaskComplete)
// 	}

// 	fn label(&self) -> String {
// 		format!("Player is choosing a player: '{}'", self.instructions)
// 	}
// }

#[derive(Clone, Debug)]
pub struct ChoosePlayerParameterTask {
	// pub parameter_type: TaskParameterType,
	pub param_name: TaskParamName,
	pub instructions: String,
	pub players: Option<Vec<ids::PlayerIndex>>,
}

impl ChoosePlayerParameterTask {
	pub fn create(param_name: TaskParamName, instructions: &'static str) -> Box<dyn PlayerTask> {
		Box::new(Self {
			param_name,
			instructions: instructions.to_string(),
			players: None,
		}) as Box<dyn PlayerTask>
	}
	pub fn one_of(
		param_name: TaskParamName, instructions: &'static str, players: Vec<ids::PlayerIndex>,
	) -> Box<dyn PlayerTask> {
		Box::new(Self {
			param_name,
			instructions: instructions.to_string(),
			players: Some(players),
		}) as Box<dyn PlayerTask>
	}
	fn get_player_indices(&self, game: &Game) -> Vec<ids::PlayerIndex> {
		if let Some(player_indices) = self.players.as_ref() {
			player_indices.to_owned()
		} else {
			(0..game.number_of_players()).collect()
		}
	}
}

impl PlayerTask for ChoosePlayerParameterTask {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[player_index].choices = Some(Choices {
			options: self
				.get_player_indices(game)
				.iter()
				.filter(|index| **index != player_index)
				.map(|victim_index| {
					TasksChoice::prepend(
						context.id_generator.generate(),
						ChoiceDisplay {
							display_type: ChoiceDisplayType::HighlightPath(DisplayPath::Player(*victim_index)),
							label: format!("choose player {}", victim_index),
						},
						vec![
							Box::new(SetParameterTask::set_player(self.param_name, *victim_index))
								as Box<dyn PlayerTask>,
						],
					)
				})
				.collect(),
			default_choice: None,
			timeline: deadlines::get_refactor_me_deadline(),
			instructions: self.instructions.to_owned(),
		});
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Player is choosing a player: '{}'", self.instructions)
	}
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum TaskParameterType {
	Player,
	Card,
	OneOf,
}

#[derive(Debug, Clone)]
pub struct SetParameterTask {
	param_name: TaskParamName,
	param_type: TaskParameterType,

	player_value: Option<ids::PlayerIndex>,
	card_value: Option<ids::CardId>,
}

impl SetParameterTask {
	pub fn set_player(param_name: TaskParamName, chosen_player: ids::PlayerIndex) -> Self {
		Self {
			param_name,
			param_type: TaskParameterType::Player,
			player_value: Some(chosen_player),
			card_value: None,
		}
	}
	pub fn set_card(param_name: TaskParamName, chosen_card: ids::CardId) -> Self {
		Self {
			param_name,
			param_type: TaskParameterType::Card,
			player_value: None,
			card_value: Some(chosen_card),
		}
	}
}

impl PlayerTask for SetParameterTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game,
		chooser_player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let tasks = &mut game.players[chooser_player_index].tasks;
		match self.param_type {
			// TODO: Can this use generics or something?
			TaskParameterType::Player => tasks.set_player_value(
				self.param_name,
				self
					.player_value
					.ok_or_else(|| SlayError::new("Expected a player value"))?,
			),
			TaskParameterType::Card => tasks.set_card_value(
				self.param_name,
				Some(
					self
						.card_value
						.ok_or_else(|| SlayError::new("Expected a card value"))?,
				),
			),
			TaskParameterType::OneOf => todo!(),
		}?;
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Set parameter {:?} to something.", self.param_name)
	}
}

#[derive(Clone, Debug)]
pub struct ClearParamsTask {}

impl ClearParamsTask {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for ClearParamsTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[player_index].tasks.clear_params();
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Clearing a players task parameter state.".to_string()
	}
}

#[derive(Clone, Debug)]
pub enum ChooseCardFilter {
	AllTopCards,
	Modifying,
}

// Rename to ChooseCardParameterTask
#[derive(Clone, Debug)]
pub struct ChooseCardFromPlayerParameterTask {
	victim_param: TaskParamName,
	card_param: TaskParamName,
	deck_path: PartialDeckPath,
	instructions: String,
	card_filter: ChooseCardFilter,
}

impl ChooseCardFromPlayerParameterTask {
	pub fn create(
		victim_param: TaskParamName, card_param: TaskParamName, deck_path: PartialDeckPath,
		instructions: &'static str,
	) -> Box<dyn PlayerTask> {
		Box::new(Self {
			victim_param,
			card_param,
			deck_path,
			instructions: instructions.to_string(),
			card_filter: ChooseCardFilter::AllTopCards,
		}) as Box<dyn PlayerTask>
	}

	pub fn modifying_cards(
		victim_param: TaskParamName, card_param: TaskParamName, deck_path: PartialDeckPath,
		instructions: &'static str,
	) -> Box<dyn PlayerTask> {
		Box::new(Self {
			victim_param,
			card_param,
			deck_path,
			instructions: instructions.to_string(),
			card_filter: ChooseCardFilter::Modifying,
		}) as Box<dyn PlayerTask>
	}

	pub fn from_party(
		victim_param: TaskParamName, card_param: TaskParamName, instructions: &'static str,
	) -> Box<dyn PlayerTask> {
		Box::new(Self {
			victim_param,
			card_param,
			deck_path: PartialDeckPath::Party, // DeckPath::Discard,
			instructions: instructions.to_string(),
			card_filter: ChooseCardFilter::AllTopCards,
		}) as Box<dyn PlayerTask>
	}

	fn create_card_choices(&self, deck: &Deck) -> Vec<CardChoiceInformation> {
		// I was over here...
		match self.card_filter {
			ChooseCardFilter::AllTopCards => deck
				.tops()
				.map(|card| CardChoiceInformation {
					card_id: card.id,
					display_path: DisplayPath::CardAt(CardPath::TopCardIn(deck.spec.path, card.id)),
					card_label: card.spec.label.to_owned(),
					perspective: card.as_perspective(),
				})
				.collect(),
			ChooseCardFilter::Modifying => deck
				.stacks()
				.flat_map(|stack| {
					stack
						.modifiers
						.iter()
						.map(|card| CardChoiceInformation {
							card_id: card.id,
							display_path: DisplayPath::CardAt(CardPath::ModifyingCardIn(
								deck.spec.path,
								stack.top.id,
								card.id,
							)),
							card_label: card.spec.label.to_owned(),
							perspective: card.as_perspective(),
						})
						.collect::<Vec<CardChoiceInformation>>()
				})
				.collect(),
		}
	}
}

impl PlayerTask for ChooseCardFromPlayerParameterTask {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game, chooser_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let victim_param = game.players[chooser_index]
			.tasks
			.get_player_value(&self.victim_param)
			.ok_or_else(|| SlayError::new("The parameter must be set."))?;

		let deck_path = self.deck_path.to_deck_path(victim_param);
		let card_choices: Vec<CardChoiceInformation> = self.create_card_choices(game.deck(deck_path));

		if card_choices.is_empty() {
			game.players[chooser_index]
				.tasks
				.set_card_value(self.card_param, None)?;
			return Ok(TaskProgressResult::TaskComplete);
		}
		game.players[chooser_index].choices = Some(Choices {
			options: card_choices
				.iter()
				.map(|card_choice| {
					TasksChoice::prepend(
						context.id_generator.generate(),
						ChoiceDisplay {
							display_type: ChoiceDisplayType::Card(card_choice.perspective.to_owned()),
							label: card_choice.card_label.to_owned(),
						},
						vec![Box::new(SetParameterTask::set_card(
							self.card_param,
							card_choice.card_id,
						)) as Box<dyn PlayerTask>],
					)
				})
				.collect(),
			default_choice: None,
			timeline: deadlines::get_refactor_me_deadline(),
			instructions: self.instructions.to_owned(),
		});
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Player is stealing a card from a specific individual.".to_string()
	}
}
