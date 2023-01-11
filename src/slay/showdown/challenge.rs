use std::rc::Rc;

use rand::rngs::ThreadRng;

use super::common::ChallengeReason;
use super::common::ModificationPath;
use super::common::RollModification;
use super::completion::CompletionTracker;
use super::consequences::RollConsequences;
use super::roll_state::list_modification_choices;
use crate::slay::choices::ChoicesPerspective;
use crate::slay::choices::{ChoicePerspective, Choices};
use crate::slay::game_context::GameBookKeeping;
use crate::slay::showdown::common::Roll;
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::state::game::Game;
use crate::slay::state::game::GameStaticInformation;
use crate::slay::{deadlines, ids};

#[derive(Debug, Clone, PartialEq)]
pub struct ChallengeRoll {
	pub player_index: ids::PlayerIndex,
	pub initial: Roll,
	pub history: Vec<RollModification>,

	path: ModificationPath,
}

#[derive(Debug, Clone)]
pub struct ChallengeState {
	pub initiator: ChallengeRoll,
	pub challenger: ChallengeRoll,
	pub completion_tracker: Option<CompletionTracker>,
	pub reason: ChallengeReason,

	consequences: RollConsequences,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChallengePerspective {
	pub initiator: ChallengeRoll,
	pub challenger: ChallengeRoll,
	pub completion_tracker: CompletionTracker,
	pub reason: ChallengeReason,
	// pub roll_total: i32,
	// pub challenger_victorious: bool,
}

impl ChallengeRoll {
	pub fn new(rng: &mut ThreadRng, player_index: ids::PlayerIndex, path: ModificationPath) -> Self {
		Self {
			initial: Roll::create_from(rng),
			history: Default::default(),
			path,
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

	pub fn roller_name(&self, statics: &Rc<GameStaticInformation>) -> String {
		statics.players[self.player_index].name.to_owned()
	}

	pub fn choices(&self, choices: &Option<ChoicesPerspective>) -> Vec<ChoicePerspective> {
		if let Some(choices) = choices {
			choices
				.options
				.iter()
				.filter(|choice| {
					choice
						.display
						.display_type
						.belongs_to_challenge_roll(self.path)
				})
				.map(|choice| choice.to_owned())
				.collect()
		} else {
			Vec::new()
		}
	}
}

// #[derive(Debug, PartialEq, Clone)]
// pub struct ChallengeRollPerspective {
// 	pub roller_name: String,
// 	pub initial: Roll,
// 	pub history: Vec<ModificationPerspective>, // Ok
// 	pub roll_total: i32,
// 	pub choices: Vec<ChoicePerspective>,
// }

impl ChallengeState {
	pub fn calculate_roll_total(&self) -> i32 {
		self.initiator.calculate_roll_total() - self.challenger.calculate_roll_total()
	}

	pub fn new(
		rng: &mut rand::rngs::ThreadRng,
		player_index: ids::PlayerIndex,
		challenger_index: ids::PlayerIndex,
		consequences: RollConsequences,
		reason: ChallengeReason,
	) -> Self {
		Self {
			initiator: ChallengeRoll::new(rng, player_index, ModificationPath::Initiator),
			challenger: ChallengeRoll::new(rng, challenger_index, ModificationPath::Challenger),
			completion_tracker: None,
			consequences,
			reason,
		}
	}

	pub fn add_modification(
		&mut self,
		modification_path: ModificationPath,
		modification: RollModification,
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
		&self,
		context: &mut GameBookKeeping,
		game: &Game,
		player_index: ids::PlayerIndex,
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

// impl ChallengeRoll {
// pub fn to_perspective(
// 	&self,
// 	game: &Game,
// 	choices: &Option<&Choices>,
// 	path: ModificationPath,
// ) -> ChallengeRoll {
// 	ChallengeRoll {
// 		roller_name: ,
// 		initial: self.initial.to_owned(),
// 		history: self
// 			.history
// 			.iter()
// 			.map(|m| m.to_perspective(game))
// 			.collect(),
// 		roll_total: self.calculate_roll_total(),
// 		choices: if let Some(choices) = choices {
// 			choices
// 				.choice_perspetives()
// 				.into_iter()
// 				.filter(|cp| match &cp.display_type {
// 					ChoiceDisplayType::Modify(modi) => path == modi.get_path(),
// 					_ => false,
// 				})
// 				.collect()
// 		} else {
// 			Vec::new()
// 		},
// 	}
// }
// }

impl ChallengeState {
	pub fn to_perspective(&self) -> ChallengePerspective {
		ChallengePerspective {
			initiator: self.initiator.to_owned(),
			challenger: self.challenger.to_owned(),
			completion_tracker: self.completion_tracker.as_ref().unwrap().to_owned(),
			reason: self.reason.to_owned(),
		}
	}
}

impl ChallengePerspective {
	pub fn calculate_roll_total(&self) -> i32 {
		// TODO: consolidate
		self.initiator.calculate_roll_total() - self.challenger.calculate_roll_total()
	}

	pub fn is_challenger_victories(&self) -> bool {
		// TODO: consolidate
		self.calculate_roll_total() <= 0
	}

	pub fn choices(&self, choices: &Option<ChoicesPerspective>) -> Vec<ChoicePerspective> {
		if let Some(choices) = choices {
			choices
				.options
				.iter()
				.filter(|choice| choice.display.display_type.belongs_to_challenge())
				.map(|x| x.to_owned())
				.collect()
		} else {
			Vec::new()
		}
	}
}
