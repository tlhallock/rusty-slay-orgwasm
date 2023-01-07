use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceInformation;
use crate::slay::choices::ChoiceLocator;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskParamName;
use crate::slay::tasks::TaskProgressResult;



#[derive(Clone, Debug)]
pub struct CardChoiceInformation {
	pub card_id: ids::CardId,
	pub display_path: DisplayPath, // This has an unnessesary clone...
	pub card_label: String,        // This has an unnessesary clone...
}

#[derive(Clone, Debug)]
pub struct ChooseCardParameterTask {
	pub param_name: TaskParamName,
	pub instructions: String,
	pub card_choices: Vec<CardChoiceInformation>,
}

impl PlayerTask for ChooseCardParameterTask {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game,
		chooser_index: ids::PlayerIndex
	) -> SlayResult<TaskProgressResult> {
		game.players[chooser_index].choices = Some(Choices {
			options: self
				.card_choices
				.iter()
				.map(|card_choice| {
					TasksChoice::prepend(
						ChoiceInformation {
							locator: ChoiceLocator {
								id: context.id_generator.generate(),
								player_index: chooser_index,
							},
							display: ChoiceDisplay {
								highlight: Some(card_choice.display_path.to_owned()),
								arrows: vec![
									// TODO
								],
								label: card_choice.card_label.to_owned(),
								roll_modification_choice: None,
							},
						},
						vec![Box::new(
							SetParameterTask::set_card(self.param_name, card_choice.card_id)
					) as Box<dyn PlayerTask>],
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
		format!(
			"Player is choosing a player: '{}'",
			self.instructions
		)
	}
}

#[derive(Clone, Debug)]
pub struct ChoosePlayerParameterTask {
	// pub parameter_type: TaskParameterType,
	pub param_name: TaskParamName,
	pub instructions: String,
}

impl PlayerTask for ChoosePlayerParameterTask {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[player_index].choices = Some(Choices {
			options: (0..game.number_of_players())
				.filter(|index| *index != player_index)
				.map(|victim_index| {
					TasksChoice::prepend(
						ChoiceInformation {
							locator: ChoiceLocator {
								id: context.id_generator.generate(),
								player_index: player_index,
							},
							display: ChoiceDisplay {
								highlight: Some(DisplayPath::Player(victim_index)),
								arrows: vec![
									// TODO
								],
								label: format!("Player {}", victim_index),
								roll_modification_choice: None,
							},
						},
						vec![Box::new(
							SetParameterTask::set_player(self.param_name, victim_index)
						) as Box<dyn PlayerTask>],
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
		format!(
			"Player is choosing a player: '{}'",
			self.instructions
		)
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
pub struct ClearParamsTask {
}

impl PlayerTask for ClearParamsTask {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[player_index].tasks.clear_params();
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!(
			"Clearing a players task parameter state.",
		)
	}
}


