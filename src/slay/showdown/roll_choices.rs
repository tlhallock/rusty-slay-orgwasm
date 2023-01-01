use crate::common::perspective::{RollModificationChoice, RollModificationChoiceType};
use crate::slay::choices::{
	self, ChoiceDisplay, ChoiceInformation, ChoiceLocator, DisplayArrow, DisplayPath,
};
use crate::slay::choices_rewrite::TasksChoice;
use crate::slay::errors::{SlayError, SlayResult};
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::{self, Card, DeckPath, Game};
use crate::slay::tasks::{MoveCardTask, PlayerTask, TaskProgressResult};
use crate::slay::{deadlines, game_context};

use super::common::{ModificationPath, RollModification};
use super::completion::{CompletionTracker, RollCompletion};
use crate::slay::showdown::current_showdown::ShowDown;

#[derive(Debug, Clone)]
pub struct ModifyRollTask {
	modifying_player_index: usize,
	modification: Option<RollModification>,
	modification_path: ModificationPath,
}
impl ModifyRollTask {
	pub fn new(
		modification: RollModification, modifying_player_index: usize, path: ModificationPath,
	) -> Self {
		Self {
			modifying_player_index,
			modification: Some(modification),
			modification_path: path,
		}
	}
}

impl PlayerTask for ModifyRollTask {
	fn make_progress(
		&mut self, context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		let modification = self
			.modification
			.take()
			.ok_or_else(|| SlayError::new("Cannot choose the same choice twice."))?;

		game
			.showdown
			.add_modification(self.modification_path, modification)?;
		let modification_task =
			game
				.showdown
				.get_modification_task(context, game, self.modifying_player_index);
		modification_task.apply(context, game);
		Ok(TaskProgressResult::TaskComplete)
	}
}

pub fn create_modify_roll_choice(
	context: &mut GameBookKeeping, _game: &Game, player_index: usize, card: &Card,
	modification_amount: i32, modification_path: &ModificationPath,
) -> TasksChoice {
	let choice_id = context.id_generator.generate();
	TasksChoice::new(
		ChoiceInformation::new(
			ChoiceLocator {
				id: choice_id,
				player_index,
			},
			choices::ChoiceDisplay {
				highlight: Some(choices::DisplayPath::CardIn(
					state::DeckPath::Hand(player_index),
					card.id,
				)),
				arrows: vec![
					choices::DisplayArrow {
						source: choices::DisplayPath::CardIn(state::DeckPath::Hand(player_index), card.id),
						destination: choices::DisplayPath::DeckAt(state::DeckPath::Discard),
					},
					choices::DisplayArrow {
						source: DisplayPath::CardIn(DeckPath::Hand(player_index), card.id),
						destination: DisplayPath::Roll(*modification_path),
					},
				],
				label: format!(
					"Use {} to modify {}'s roll by {}",
					card.id, "somebody", player_index,
				),
				roll_modification_choice: Some(RollModificationChoice {
					choice_id,
					choice_type: RollModificationChoiceType::from_card(&card.spec, modification_amount),
				}),
			},
		),
		vec![
			Box::new(MoveCardTask {
				source: state::DeckPath::Hand(player_index),
				destination: state::DeckPath::Discard,
				card_id: card.id,
			}) as Box<dyn PlayerTask>,
			Box::new(ModifyRollTask::new(
				RollModification {
					modifying_player_index: player_index,
					card_id: card.id,
					modification_amount,
				},
				player_index,
				*modification_path,
			)) as Box<dyn PlayerTask>,
		],
	)
}

/*

let move_card =  ;
				*/

// #[derive(Clone, Debug)]
// pub struct ModifyRollChoice {
//     modification: Option<RollModification>,
//     choice_information: choices::ChoiceInformation,
//     modification_path: ModificationPath,
// }

// impl ModifyRollChoice {
//     pub fn new(
//         modification: RollModification,
//         choice_information: choices::ChoiceInformation,
//         path: ModificationPath,
//     ) -> Self {
//         Self {
//             modification: Some(modification),
//             choice_information,
//             modification_path: path,
//         }
//     }

//     pub fn new2(
//         modification: RollModification,
//         locator: choices::ChoiceLocator,
//         modification_path: ModificationPath,
//     ) -> Self {
//         let modifying_player_index = modification.modifying_player_index;
//         let card_id = modification.card_id;
//         let modification_amount = modification.modification_amount;

//         Self {
//             modification_path,
//             modification: Some(modification),
//             choice_information:
//     }
// }

// impl choices::Choice for ModifyRollChoice {
//     fn select(
//         &mut self,
//         context: &mut game_context::GameBookKeeping,
//         game: &mut state::Game,
//     ) -> SlayResult<()> {
//         let modification = self
//             .modification
//             .take()
//             .ok_or_else(|| SlayError::new("Cannot choose the same choice twice."))?;
//         state_modifiers::transfer_a_top_card(
//             modification.card_id,
//             &mut game.players[modification.modifying_player_index].hand,
//             &mut game.draw,
//         )?;

//         let modifying_player_index = modification.modifying_player_index;
//         game.showdown
//             .add_modification(self.modification_path, modification)?;
//         let modification_task =
//             game.showdown
//                 .get_modification_task(context, game, modifying_player_index);
//         modification_task.apply(context, game);
//         Ok(())
//     }

//     fn get_choice_information(&self) -> &choices::ChoiceInformation {
//         &self.choice_information
//     }
// }

#[derive(Debug, Clone)]
pub struct SetCompleteTask {
	player_index: usize,
	persist: RollCompletion,
}

impl SetCompleteTask {
	pub fn new(player_index: usize, persist: RollCompletion) -> Self {
		Self {
			player_index,
			persist,
		}
	}
}

impl PlayerTask for SetCompleteTask {
	fn make_progress(
		&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		let player_index = self.player_index;
		game
			.showdown
			.set_player_completion(player_index, self.persist)?;
		// .as_mut()
		// .ok_or_else(|| SlayError::new("No show down"))?
		// .tracker_mut()
		// .set_player_completion(player_index, self.persist);
		game.players[player_index].choices = None;
		Ok(TaskProgressResult::TaskComplete)
	}
}

fn create_set_complete_choice(
	locator: ChoiceLocator, persist: RollCompletion, label: String,
) -> TasksChoice {
	let player_index = locator.player_index;
	let choice_id = locator.id;
	TasksChoice::new(
		choices::ChoiceInformation {
			locator,
			display: choices::ChoiceDisplay {
				label,
				roll_modification_choice: Some(RollModificationChoice {
					choice_id,
					choice_type: RollModificationChoiceType::Nothing(persist),
				}),
				..Default::default()
			},
		},
		vec![Box::new(SetCompleteTask::new(player_index, persist)) as Box<dyn PlayerTask>],
	)
}

pub fn create_set_completion_done(locator: ChoiceLocator) -> TasksChoice {
	create_set_complete_choice(locator, RollCompletion::AllDone, "Do nothing.".to_string())
}

pub fn create_set_completion_until_modification(locator: ChoiceLocator) -> TasksChoice {
	create_set_complete_choice(
		locator,
		RollCompletion::DoneUntilModification,
		"Don't modify this roll unless someone else does.".to_string(),
	)
}

pub fn create_challenge_choice(
	locator: ChoiceLocator, challenge_card_id: ids::CardId,
) -> TasksChoice {
	let player_index = locator.player_index;
	TasksChoice::new(
		ChoiceInformation {
			locator,
			display: ChoiceDisplay {
				highlight: Some(DisplayPath::CardIn(
					DeckPath::Hand(player_index),
					challenge_card_id,
				)),
				arrows: vec![DisplayArrow {
					source: DisplayPath::CardIn(DeckPath::Hand(player_index), challenge_card_id),
					destination: DisplayPath::DeckAt(DeckPath::Discard),
				}],
				label: "Challenge that action...".to_string(),
				roll_modification_choice: None,
			},
		},
		vec![
			Box::new(MoveCardTask {
				source: state::DeckPath::Hand(player_index),
				destination: state::DeckPath::Discard,
				card_id: challenge_card_id,
			}) as Box<dyn PlayerTask>,
			Box::new(ChallengeTask::new(player_index)) as Box<dyn PlayerTask>,
		],
	)
}

#[derive(Debug, Clone)]
struct ChallengeTask {
	challenging_player_index: usize,
}
impl ChallengeTask {
	pub fn new(challenging_player_index: usize) -> Self {
		Self {
			challenging_player_index,
		}
	}
}
impl PlayerTask for ChallengeTask {
	fn make_progress(
		&mut self, context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		let offer = game.showdown.take_current_offer()?;
		let mut challenge = offer.to_challenge(&mut context.rng, self.challenging_player_index)?;
		challenge.completion_tracker = Some(CompletionTracker::new(
			game.number_of_players(),
			deadlines::get_challenge_deadline(),
		));
		challenge.assign_all_choices(context, game);
		game.showdown.challenge(challenge);
		Ok(TaskProgressResult::ChoicesAssigned)
	}
}
