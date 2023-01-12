use crate::slay::errors;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::task_params::TaskParams;

use core::fmt::Debug;
use std::collections::VecDeque;
use std::io::BufWriter;
use std::io::Write;

pub enum TaskProgressResult {
	NothingDone,
	ProgressMade,
	TaskComplete,
	/*
	If TaskComplete, the task is     REMOVED from the queue, and the next Task IS     tried.
	If NothingDone,  the task is NOT REMOVED from the queue, and the next Task IS NOT tried.
	If ProgressMade, the task is NOT REMOVED from the queue, and the next Task IS     tried.

	Note: No need for another result for when the task is complete, but the next task should not be tried.
	In this scenario, choices are assigned, and no more tasks are attempted anyway.



	Dude, if we could make them all be TaskComplete, then we could always just remove 'em!!!
	*/
}

dyn_clone::clone_trait_object!(PlayerTask);

pub trait PlayerTask: Debug + dyn_clone::DynClone {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult>;

	// TODO: Make this just extend Summarizable...
	fn label(&self) -> String;
}

#[derive(Debug, Default, Clone)]
pub struct PlayerTasks {
	// Maybe this should be a stack?
	// I don't like this part of the code. Seems complicated.
	// Help? (interior mutability?)
	// Each task is mutable while being performed (maybe it updates some params..),
	// and the game is mutable (obvioiusly) and the task is part of the game.
	// Due to ownership rules, we cannot pass a mutable game into the task, while
	// the task is still within the game.
	// Therefore, we remove the task, do the task, then put it back.
	// While doing the task, the task may prepend more tasks.
	// These have to go before the current task, after it is put back.
	prepend: Vec<Box<dyn PlayerTask>>, // if we were worried about efficiency, this would not be a vec...
	upcoming: VecDeque<Box<dyn PlayerTask>>,
	current: Option<Box<dyn PlayerTask>>,
	params: TaskParams,
}

impl Summarizable for PlayerTasks {
	fn summarize<W: Write>(
		&self,
		f: &mut BufWriter<W>,
		indentation_level: u32,
	) -> Result<(), std::io::Error> {
		if self.upcoming.is_empty() && self.current.is_none() {
			return Ok(());
		}

		for _ in 0..indentation_level {
			write!(f, "  ")?;
		}
		write!(f, "tasks:")?;
		writeln!(f)?;

		for _ in 0..indentation_level + 1 {
			write!(f, "  ")?;
		}
		write!(f, "upcoming: ")?;
		for task in self.prepend.iter() {
			write!(f, "{}, ", task.label())?;
		}
		if let Some(task) = self.current.as_ref() {
			write!(f, "{}, ", task.label())?;
		}
		for task in self.upcoming.iter() {
			write!(f, "{}, ", task.label())?;
		}
		writeln!(f)?;

		self.params.summarize(f, indentation_level + 1)?;
		Ok(())
	}
}

impl PlayerTasks {
	pub fn new(tasks: Vec<Box<dyn PlayerTask>>) -> Self {
		Self {
			upcoming: VecDeque::from(tasks),
			current: None,
			params: Default::default(),
			prepend: Default::default(),
		}
	}

	pub fn take_from(&mut self, to_take: &mut Vec<Box<dyn PlayerTask>>) {
		self.upcoming.extend(to_take.drain(..));
	}
	pub fn prepend_from(&mut self, to_take: &mut Vec<Box<dyn PlayerTask>>) {
		// Should this have been a stack all along?
		self.prepend.append(to_take);
		// log::info!("About to prepend {:?} to {:?}")
		// while !to_take.is_empty() {
		// 	let task = to_take.remove(0);
		// 	self.upcoming.push_front(task);
		// }
	}
	pub fn prepend(&mut self, next_task: Box<dyn PlayerTask>) {
		self.prepend.push(next_task);
	}

	pub fn put_current_task_back(&mut self, task: Box<dyn PlayerTask>) -> SlayResult<()> {
		// reviewer: How do you make this a one liner?
		if self.current.is_some() {
			return Err(errors::SlayError::new(
				"The current action should have been taken out right now.",
			));
		}
		self.current = Some(task);
		Ok(())
	}

	fn ensure_rotated(&mut self) {
		if self.prepend.is_empty() {
			return;
		}
		if let Some(task) = self.current.take() {
			self.upcoming.push_front(task);
		}
		while !self.prepend.is_empty() {
			let last_index = self.prepend.len() - 1;
			let task = self.prepend.remove(last_index);
			self.upcoming.push_front(task);
		}
	}

	pub fn take_current_task(&mut self) -> Option<Box<dyn PlayerTask>> {
		self.ensure_rotated();
		self.current.take().or_else(|| {
			// Initialize the task, if need be...
			self.upcoming.pop_front()
		})
	}

	pub(crate) fn set_player_value(
		&mut self,
		param_name: TaskParamName,
		player_index: ids::PlayerIndex,
	) -> SlayResult<()> {
		if self
			.params
			.players
			.insert(param_name, player_index)
			.is_some()
		{
			Err(SlayError::new("Overwriting a parameter value."))
		} else {
			Ok(())
		}
	}

	pub(crate) fn set_card_value(
		&mut self,
		param_name: TaskParamName,
		card_id: Option<ids::CardId>,
	) -> SlayResult<()> {
		if let Some(previous) = self.params.cards.insert(param_name, card_id) {
			log::error!(
				"Trying to set parameter {:?} to {:?}, but it was already set to {:?}",
				param_name,
				card_id,
				previous
			);
			Err(SlayError::new("Overwriting a parameter value."))
		} else {
			Ok(())
		}
	}
	pub(crate) fn get_card_value(&self, param_name: &TaskParamName) -> Option<Option<ids::CardId>> {
		self.params.cards.get(param_name).copied()
	}

	pub(crate) fn get_player_value(&self, param_name: &TaskParamName) -> Option<ids::PlayerIndex> {
		self.params.players.get(param_name).copied()
	}

	pub fn clear_params(&mut self) {
		self.params.clear();
	}
}

pub(crate) fn continue_tasks(
	context: &mut GameBookKeeping,
	game: &mut Game,
	player_index: ids::PlayerIndex,
) -> SlayResult<TaskProgressResult> {
	/*
	
	NOTE:
		Actions can assign choices to other players aside from the current player.
		This check of choices.is_some() is correct (I think) but tricker to think through.
	*/
	let mut result = TaskProgressResult::NothingDone;
	loop {
		if game.players[player_index].choices.is_some() {
			log::debug!("Player {} already has choices", player_index);
			return Ok(TaskProgressResult::NothingDone);
		}
		if let Some(mut task) = game.take_current_task(player_index) {
			let label = task.as_ref().label();
			log::info!("Took task '{}'", label);
			match task.make_progress(context, game, player_index)? {
				TaskProgressResult::TaskComplete => {
					result = TaskProgressResult::ProgressMade;
					log::info!("Task '{}' complete", label);
				}
				TaskProgressResult::ProgressMade => {
					game.players[player_index].put_current_task_back(task)?;
					result = TaskProgressResult::ProgressMade;
					log::info!("Returning to '{}' later", label);
				}
				TaskProgressResult::NothingDone => {
					log::info!("Nothing to be done for task '{}'", label);
					game.players[player_index].put_current_task_back(task)?;
					return Ok(result);
				}
			};
		} else {
			return Ok(result);
		}
	}
}
