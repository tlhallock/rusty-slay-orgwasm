use crate::slay::choices;
use crate::slay::deadlines;
use crate::slay::errors;
use crate::slay::errors::SlayResult;
use crate::slay::game_context;
use crate::slay::ids;
use crate::slay::modifiers;
use crate::slay::state;

use core::fmt::Debug;

use std::collections::HashMap;
use std::collections::VecDeque;

use crate::slay::choices::ChoiceDisplay;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::state::Game;

use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceInformation;
use crate::slay::choices::ChoiceLocator;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::state::DeckPath;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskSpec {
	Sacrifice(u32),
	Discard(u32),
	ReceiveModifier(modifiers::PlayerModifier),
	Draw(u32),
}

impl TaskSpec {
	pub fn to_task(&self, player_index: usize) -> Box<dyn PlayerTask> {
		match &self {
			TaskSpec::Sacrifice(num) => Box::new(Sacrifice::new(*num, player_index)),
			TaskSpec::Discard(num) => Box::new(Discard::new(player_index, *num)),
			TaskSpec::ReceiveModifier(modifier) => {
				Box::new(ReceiveModifier::new(player_index, *modifier))
			}
			TaskSpec::Draw(num) => Box::new(Draw::new(player_index, *num)),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TaskParamName {
	Victim,
}

#[derive(Debug, Default, Clone)]
pub struct TaskParams {
	pub players: HashMap<TaskParamName, ids::PlayerId>,
	pub cards: HashMap<TaskParamName, ids::CardId>,
	pub index: HashMap<TaskParamName, usize>,
}

pub enum TaskProgressResult {
	NothingDone,
	ProgressMade,
	TaskComplete,
	// ChoicesAssigned,
	// ChoicesAlreadyAssigned,
}

dyn_clone::clone_trait_object!(PlayerTask);

pub trait PlayerTask: Debug + dyn_clone::DynClone {
	fn make_progress(
		&mut self, context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<TaskProgressResult>;

	fn label(&self) -> String;
}

#[derive(Debug, Default, Clone)]
pub struct PlayerTasks {
	// tasks: VecDeque<PlayerTask>,
	upcoming: VecDeque<Box<dyn PlayerTask>>,
	current: Option<Box<dyn PlayerTask>>,
	params: TaskParams,
}

impl PlayerTasks {
	pub fn new(tasks: Vec<Box<dyn PlayerTask>>) -> Self {
		Self {
			upcoming: VecDeque::from(tasks),
			current: None,
			params: Default::default(),
		}
	}

	pub fn take_from(&mut self, to_take: &mut Vec<Box<dyn PlayerTask>>) {
		self.upcoming.extend(to_take.drain(..));
	}
	pub fn prepend_from(&mut self, to_take: &mut Vec<Box<dyn PlayerTask>>) {
		// Should this have been a stack all along?
		while !to_take.is_empty() {
			let task = to_take.remove(0);
			self.upcoming.push_front(task);
		}
	}

	pub fn put_current_task_back(&mut self, task: Box<dyn PlayerTask>) -> SlayResult<()> {
		// reviewer: How do you make this a one liner?
		if self.current.is_some() {
			return Err(errors::SlayError::new(
				"The current action should be taken out right now.",
			));
		}
		self.current = Some(task);
		Ok(())
	}
	pub fn take_current_task(&mut self) -> Option<Box<dyn PlayerTask>> {
		self.current.take().or_else(|| {
			// Initialize the task, if need be...
			self.upcoming.pop_front()
		})
	}
}

#[derive(Debug, Clone)]
struct Sacrifice {
	num: u32,
	player_index: usize,
}

impl Sacrifice {
	pub fn new(num: u32, player_index: usize) -> Self {
		Self { num, player_index }
	}
}

// TODO: This could use the move card task?
// #[derive(Debug, Clone)]
// struct MoveCardChoice {
// 	source: state::DeckPath,
// 	destination: state::DeckPath,
// 	card_id: ids::CardId,
// 	choice_information: choices::ChoiceInformation,
// }

// impl choices::Choice for MoveCardChoice {
// 	fn select(
// 		&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game,
// 	) -> SlayResult<()> {
// 		game.move_card(self.source, self.destination, self.card_id)
// 	}

// 	fn get_choice_information(&self) -> &choices::ChoiceInformation {
// 		&self.choice_information
// 	}
// }

// fn card_is_sacrificable(stack: &state::Stack) -> bool {
//   true
// }

impl PlayerTask for Sacrifice {
	fn make_progress(
		&mut self, context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		if self.num == 0 {
			return Ok(TaskProgressResult::TaskComplete);
		}

		let party = &game.players[self.player_index].party;
		let mut options: Vec<TasksChoice> = party
			.stacks
			.iter()
			// .filter(card_is_sacrificable)
			.map(|s| {
				TasksChoice::new(
					choices::ChoiceInformation {
						locator: choices::ChoiceLocator {
							id: context.id_generator.generate(),
							player_index: self.player_index,
						},
						display: ChoiceDisplay {
							label: format!("Sacrifice {}.", s.top.label()),
							highlight: Some(choices::DisplayPath::CardIn(
								state::DeckPath::Hand(self.player_index),
								s.top.id,
							)),
							arrows: vec![choices::DisplayArrow {
								source: choices::DisplayPath::CardIn(
									state::DeckPath::Hand(self.player_index),
									s.top.id,
								),
								destination: choices::DisplayPath::DeckAt(state::DeckPath::Discard),
							}],
							roll_modification_choice: None,
						},
					},
					vec![Box::new(MoveCardTask {
						source: state::DeckPath::Party(self.player_index),
						destination: state::DeckPath::Discard,
						card_id: s.top.id,
					})],
				)
			})
			.collect();

		if options.len() == self.num as usize {
			for option in options.iter_mut() {
				option.select(context, game)?;
			}
			return Ok(TaskProgressResult::TaskComplete);
		}

		if options.is_empty() {
			return Ok(TaskProgressResult::TaskComplete);
		}

		let default_choice = options[0].get_choice_information().get_id();
		game.players[self.player_index].choices = Some(choices::Choices {
			instructions: "Choose a card to sacrifice.".to_string(),
			options,
			default_choice,
			timeline: deadlines::get_sacrifice_deadline(),
		});

		self.num -= 1;
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!(
			"Player {} is sacrificing {} heros.",
			self.player_index, self.num
		)
	}
}

#[derive(Debug, Clone)]
struct ReceiveModifier {
	player_index: usize,
	modifier: PlayerModifier,
}

impl ReceiveModifier {
	pub fn new(player_index: usize, modifier: PlayerModifier) -> Self {
		Self {
			player_index,
			modifier,
		}
	}
}

impl PlayerTask for ReceiveModifier {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game,
	) -> SlayResult<TaskProgressResult> {
		game.players[self.player_index]
			.buffs
			.add_forever(self.modifier.to_owned());
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!(
			"Player {} is receiving modifier {:?}",
			self.player_index, self.modifier
		)
	}
}

#[derive(Debug, Clone)]
struct Discard {
	player_index: usize,
	num: u32,
}

impl Discard {
	pub fn new(player_index: usize, num: u32) -> Self {
		Self { player_index, num }
	}
}

impl PlayerTask for Discard {
	fn make_progress(
		&mut self, context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		if self.num == 0 {
			return Ok(TaskProgressResult::TaskComplete);
		}
		self.num -= 1;
		let options: Vec<TasksChoice> = game.players[self.player_index]
			.hand
			.stacks
			.iter()
			.map(|stack| {
				TasksChoice::prepend(
					ChoiceInformation {
						locator: ChoiceLocator {
							id: context.id_generator.generate(),
							player_index: self.player_index,
						},
						display: ChoiceDisplay {
							highlight: Some(DisplayPath::CardIn(
								DeckPath::Hand(self.player_index),
								stack.top.id,
							)),
							arrows: vec![], // Todo
							label: format!("Discard {}", stack.top.spec.label),
							roll_modification_choice: None,
						},
					},
					vec![Box::new(MoveCardTask {
						source: DeckPath::Hand(self.player_index),
						destination: DeckPath::Discard,
						card_id: stack.top.id,
					}) as Box<dyn PlayerTask>],
				)
			})
			.collect();

		if options.is_empty() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let default_choice = options[0].get_choice_information().locator.id;

		game.players[self.player_index].choices = Some(Choices::new(
			options,
			default_choice,
			deadlines::get_discard_deadline(),
			"Choose a card to discard.".to_owned(),
		));
		Ok(TaskProgressResult::ProgressMade)
	}
	fn label(&self) -> String {
		format!(
			"Player {} is discarding {} cards",
			self.player_index, self.num
		)
	}
}

#[derive(Debug, Clone)]
struct Draw {
	player_index: usize,
	num: u32,
}

impl Draw {
	pub fn new(player_index: usize, num: u32) -> Self {
		Self { player_index, num }
	}
}

impl PlayerTask for Draw {
	fn make_progress(
		&mut self, _context: &mut game_context::GameBookKeeping, _game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		todo!()
	}
	fn label(&self) -> String {
		format!(
			"Player {} is drawing {} cards.",
			self.player_index, self.num
		)
	}
}

pub(crate) fn continue_tasks(
	context: &mut game_context::GameBookKeeping, game: &mut state::Game, player_index: usize,
) -> SlayResult<TaskProgressResult> {
	let mut result = TaskProgressResult::NothingDone;
	loop {
		if game.players[player_index].choices.is_some() {
			log::debug!("Player {} already has choices", player_index);
			return Ok(TaskProgressResult::NothingDone);
		}
		if let Some(mut task) = game.take_current_task(player_index) {
			let label = task.as_ref().label();
			log::debug!("Took task '{}'", label);
			match task.make_progress(context, game)? {
				TaskProgressResult::TaskComplete => {
					result = TaskProgressResult::ProgressMade;
					log::debug!("Task '{}' complete", label);
				}
				TaskProgressResult::ProgressMade => {
					game.players[player_index].put_current_task_back(task)?;
					log::debug!("Returning to '{}' later", label);
					return Ok(TaskProgressResult::ProgressMade);
				}
				TaskProgressResult::NothingDone => {
					log::debug!("Nothing to be done for task '{}'", label);
					game.players[player_index].put_current_task_back(task)?;
					return Ok(result);
				}
			};
		} else {
			return Ok(result);
		}
	}
}

#[derive(Debug, Clone)]
pub struct MoveCardTask {
	pub source: state::DeckPath,
	pub destination: state::DeckPath,
	pub card_id: ids::CardId,
}

impl PlayerTask for MoveCardTask {
	fn make_progress(
		&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		game.move_card(self.source, self.destination, self.card_id)?;
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!(
			"Moving {} from {:?} to {:?}",
			self.card_id, self.source, self.destination
		)
	}
}

// #[derive(Debug, Clone)]
// pub struct RollForAbilityTask {
//     pub player_index: usize,
//     pub card_id: ids::CardId,
// }

// impl PlayerTask for RollForAbilityTask {
//     fn make_progress(
//         &mut self,
//         context: &mut game_context::GameBookKeeping,
//         game: &mut state::Game,
//     ) -> SlayResult<TaskProgressResult> {
//     }
// }

#[derive(Debug, Clone)]
pub struct UseAbilityTask {
	pub deck_path: state::DeckPath,
	pub card_id: ids::CardId,
}

impl PlayerTask for UseAbilityTask {
	fn make_progress(
		&mut self, _context: &mut game_context::GameBookKeeping, _game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		// do the ability!!
		// Implement it!
		log::info!("We got here!");
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("I wish I were this far...")
	}
}
