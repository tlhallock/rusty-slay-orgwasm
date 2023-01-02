use crate::slay::choices::Choices;

use crate::slay::game_context::GameBookKeeping;
use crate::slay::state::Game;
use crate::slay::{deadlines, game_context};

use crate::slay::showdown::common::Roll;

use super::common::{ChallengeReason, ModificationPath, RollModification};
use super::completion::CompletionTracker;
use super::consequences::RollConsequences;
use super::roll_state::list_modification_choices;
use crate::slay::showdown::current_showdown::ShowDown;

#[derive(Debug, Clone)]
pub struct ChallengeRoll {
	pub initial: Roll,
	pub history: Vec<RollModification>,
	pub player_index: usize,
}

impl ChallengeRoll {
	pub fn new(rng: &mut rand::rngs::ThreadRng, player_index: usize) -> Self {
		Self {
			initial: Roll::create_from(rng),
			history: Default::default(),
			player_index,
		}
	}

	pub fn calculate_roll_total(&self) -> i32 {
		self.initial.die1 as i32
			+ self.initial.die2 as i32
			+ self
				.history
				.iter()
				.map(|h| h.modification_amount)
				.sum::<i32>()
	}
}

#[derive(Debug, Clone)]
pub struct ChallengeState {
	pub initiator: ChallengeRoll,
	pub challenger: ChallengeRoll,
	pub completion_tracker: Option<CompletionTracker>,
	pub reason: ChallengeReason,

	consequences: RollConsequences,
}

impl ChallengeState {
	pub fn calculate_roll_total(&self) -> i32 {
		self.initiator.calculate_roll_total() - self.challenger.calculate_roll_total()
	}

	pub fn new(
		rng: &mut rand::rngs::ThreadRng, player_index: usize, challenger_index: usize,
		consequences: RollConsequences, reason: ChallengeReason,
	) -> Self {
		Self {
			initiator: ChallengeRoll::new(rng, player_index),
			challenger: ChallengeRoll::new(rng, challenger_index),
			completion_tracker: None,
			consequences,
			reason,
		}
	}

	pub fn add_modification(
		&mut self, modification_path: ModificationPath, modification: RollModification,
	) {
		self.tracker_mut().timeline = deadlines::get_challenge_deadline();
		match modification_path {
			ModificationPath::Roll => panic!(),
			// Err(SlayError::new(
			//     "Expected to modify a roll, found a challenge.",
			// )),
			ModificationPath::Challenger => {
				self.challenger.history.push(modification);
			}
			ModificationPath::Initiator => {
				self.initiator.history.push(modification);
			}
		}
	}
}

impl ShowDown for ChallengeState {
	fn tracker(&self) -> &CompletionTracker {
		self.completion_tracker.as_ref().unwrap()
	}

	fn tracker_mut(&mut self) -> &mut CompletionTracker {
		self.completion_tracker.as_mut().unwrap()
	}

	fn create_choice_for(
		&self, context: &mut GameBookKeeping, game: &Game, player_index: usize,
	) -> Choices {
		let default_choice = context.id_generator.generate();
		Choices {
			instructions: "Choose whether to modify the current challenge.".to_string(),
			default_choice,
			timeline: self.tracker().timeline.to_owned(),
			options: list_modification_choices(
				context,
				game,
				player_index,
				default_choice,
				vec![ModificationPath::Challenger, ModificationPath::Initiator],
			),
		}
	}

	fn finish(&mut self, _context: &mut game_context::GameBookKeeping, game: &mut Game) {
		let roll_sum = self.calculate_roll_total();
		self.consequences.apply_roll_sum(game, roll_sum);
	}
}
