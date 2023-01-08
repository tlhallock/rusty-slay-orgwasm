use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;
use crate::slay::tasks::PlayerTask;

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

#[derive(Debug, Clone)]
pub enum ConsequenceName {}

#[derive(Debug, Clone)]
pub struct RollConsequence {
	pub condition: Condition,
	pub tasks: Vec<Box<dyn PlayerTask>>,
}

#[derive(Debug, Clone)]
pub struct RollConsequences {
	pub success: RollConsequence,
	pub loss: Option<RollConsequence>,
}

impl RollConsequences {
	pub fn new(success: RollConsequence, loss: Option<RollConsequence>) -> Self {
		Self { success, loss }
	}

	pub fn take_tasks_for(&mut self, roll_sum: i32) -> Vec<Box<dyn PlayerTask>> {
		let mut ret = Vec::new();
		if self.success.condition.applies_to(roll_sum) {
			ret.append(&mut self.success.tasks);
		}
		if let Some(consequence) = self.loss.as_mut() {
			if consequence.condition.applies_to(roll_sum) {
				ret.append(&mut consequence.tasks);
			}
		}
		ret
	}

	pub(crate) fn proceed(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) {
		self.apply_roll_sum(game, 1, player_index);
	}

	pub(crate) fn apply_roll_sum(
		&mut self, game: &mut Game, roll_sum: i32, player_index: ids::PlayerIndex,
	) {
		game.players.iter_mut().for_each(|p| p.choices = None);
		// TODO: An extra copy here...
		let mut tasks = self.take_tasks_for(roll_sum);
		game.players[player_index].tasks.take_from(&mut tasks);
	}
}
