use crate::slay::game_context;
use crate::slay::state;
use crate::slay::tasks;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::PlayerTasks;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
	LE,
	GE,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
	pub cmp: Comparison,
	pub threshold: i32,
}

impl Condition {
	pub fn challenge_sustained() -> Self {
		Self {
			cmp: Comparison::LE,
			threshold: 0,
		}
	}
	pub fn challenge_denied() -> Self {
		Self {
			cmp: Comparison::GE,
			threshold: 1,
		}
	}

	pub fn applies_to(&self, roll_sum: i32) -> bool {
		match self.cmp {
			Comparison::GE => roll_sum >= self.threshold,
			Comparison::LE => roll_sum <= self.threshold,
		}
	}
}

impl Condition {
	pub fn ge(threshold: i32) -> Self {
		Self {
			cmp: Comparison::GE,
			threshold,
		}
	}
	pub fn le(threshold: i32) -> Self {
		Self {
			cmp: Comparison::LE,
			threshold,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollConsequenceSpec {
	pub condition: Condition,
	pub tasks: Vec<tasks::TaskSpec>,
}

impl RollConsequenceSpec {
	fn tasks(&self, player_index: usize) -> Vec<Box<dyn tasks::PlayerTask>> {
		self.tasks.iter().map(|t| t.to_task(player_index)).collect()
	}

	pub fn to_smh(&self, player_index: usize) -> RollConsequenceRenameMe {
		RollConsequenceRenameMe {
			condition: self.condition.to_owned(),
			tasks: self.tasks(player_index),
		}
	}
}

#[derive(Debug, Clone)]
pub struct RollConsequenceRenameMe {
	pub condition: Condition,
	pub tasks: Vec<Box<dyn tasks::PlayerTask>>,
}

#[derive(Debug, Clone)]
pub struct RollConsequences {
	player_index: usize,
	consequences: Vec<RollConsequenceRenameMe>,
}

impl RollConsequences {
	pub fn new(player_index: usize, consequences: Vec<RollConsequenceRenameMe>) -> Self {
		Self {
			player_index,
			consequences,
		}
	}

	pub fn take(&mut self) -> Self {
		Self {
			player_index: self.player_index,
			consequences: self.consequences.drain(..).collect(),
		}
	}

	pub fn take_tasks(&mut self, roll_sum: i32) -> Vec<Box<dyn PlayerTask>> {
		self
			.consequences
			.drain(..)
			.filter(|c| c.condition.applies_to(roll_sum))
			.flat_map(|c| c.tasks)
			.collect() // reviewer: Shouldn't need to collect here.
	}

	pub(crate) fn proceed(
		&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) {
		self.apply_roll_sum(game, 1);
	}

	pub(crate) fn apply_roll_sum(&mut self, game: &mut state::Game, roll_sum: i32) {
		game.players.iter_mut().for_each(|p| p.choices = None);
		// TODO: An extra copy here...
		let mut tasks = self.take_tasks(roll_sum);
		game.players[self.player_index].tasks.take_from(&mut tasks);
	}
}
