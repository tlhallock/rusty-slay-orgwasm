use crate::slay::choices::{ChoicePerspective, Choices, ChoicesPerspective, DisplayPath};

use crate::slay::deadlines::Timeline;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::state::game::Game;
use crate::slay::{deadlines, ids};

use crate::slay::showdown::common::Roll;

use super::common::ChallengeReason;
use super::common::ModificationPath;
use super::common::{ModificationPerspective, RollModification};
use super::completion::{CompletionTracker, PlayerCompletionPerspective};
use super::consequences::RollConsequences;
use super::roll_state::list_modification_choices;
use crate::slay::showdown::current_showdown::ShowDown;

#[derive(Debug, Clone)]
pub struct ChallengeRoll {
	pub initial: Roll,
	pub history: Vec<RollModification>,
	pub player_index: ids::PlayerIndex,
}

impl ChallengeRoll {
	pub fn new(rng: &mut rand::rngs::ThreadRng, player_index: ids::PlayerIndex) -> Self {
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
		rng: &mut rand::rngs::ThreadRng, player_index: ids::PlayerIndex,
		challenger_index: ids::PlayerIndex, consequences: RollConsequences, reason: ChallengeReason,
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
		&self, context: &mut GameBookKeeping, game: &Game, player_index: ids::PlayerIndex,
	) -> Choices {
		let default_choice = context.id_generator.generate();
		Choices {
			instructions: "Choose whether to modify the current challenge.".to_string(),
			default_choice: Some(default_choice),
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

	fn finish(&mut self, _context: &mut GameBookKeeping, game: &mut Game) {
		let roll_sum = self.calculate_roll_total();
		self
			.consequences
			.apply_roll_sum(game, roll_sum, self.initiator.player_index);
	}
}

impl ChallengeRoll {
	pub fn to_perspective(
		&self, game: &Game, choices: &Option<ChoicesPerspective>, display_path: DisplayPath,
	) -> ChallengeRollPerspective {
		ChallengeRollPerspective {
			id: 0, // Need to fill this in again?
			roller_name: game.players[self.player_index].name.to_owned(),
			initial: self.initial.to_owned(),
			history: self
				.history
				.iter()
				.map(|m| m.to_perspective(game))
				.collect(),
			roll_total: self.calculate_roll_total(),
			choices: choices
				.iter()
				.map(|choices| {
					choices
						.actions
						.iter()
						.filter(|choice| choice.highlight_path.iter().any(|dp| *dp == display_path))
						.map(|o| o.to_owned())
						.collect::<Vec<ChoicePerspective>>()
				})
				.flatten()
				.collect(),
		}
	}
}

impl ChallengeState {
	pub fn to_perspective(
		&self, game: &Game, choices: &Option<ChoicesPerspective>,
	) -> ChallengePerspective {
		ChallengePerspective {
			completions: self.tracker().to_perspective(game),
			success: false,
			timeline: self.tracker().timeline.to_owned(),
			reason: self.reason.to_owned(),
			initiator: self.initiator.to_perspective(
				game,
				choices,
				DisplayPath::Roll(ModificationPath::Initiator),
			),
			challenger: self.challenger.to_perspective(
				game,
				choices,
				DisplayPath::Roll(ModificationPath::Challenger),
			),
			roll_total: self.calculate_roll_total(),
			choices: choices
				.iter()
				.map(|choices| {
					choices
						.actions
						.iter()
						.filter(|choice| choice.highlight_path.is_none())
						.map(|o| o.to_owned())
						.collect::<Vec<ChoicePerspective>>()
				})
				.flatten()
				.collect(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChallengePerspective {
	pub initiator: ChallengeRollPerspective,
	pub challenger: ChallengeRollPerspective,
	pub completions: Vec<PlayerCompletionPerspective>,
	pub success: bool,
	pub timeline: Timeline,
	pub reason: ChallengeReason,
	pub choices: Vec<ChoicePerspective>,
	pub roll_total: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChallengeRollPerspective {
	id: ids::RollId,
	pub roller_name: String,
	pub initial: Roll,
	pub history: Vec<ModificationPerspective>,
	pub roll_total: i32,
	pub choices: Vec<ChoicePerspective>,
}
