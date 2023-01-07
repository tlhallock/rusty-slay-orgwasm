use crate::slay::choices::{
	self, ChoiceDisplay, ChoiceInformation, ChoiceLocator, DisplayArrow, DisplayPath, TasksChoice,
};
use crate::slay::errors::{SlayError, SlayResult};
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks::{MoveCardTask, PlayerTask, TaskProgressResult};
use crate::slay::{deadlines, game_context};

use super::common::{
	ModificationPath, RollModification, RollModificationChoice, RollModificationChoiceType,
};
use super::completion::{CompletionTracker, RollCompletion};
use crate::slay::showdown::current_showdown::ShowDown;

#[derive(Debug, Clone)]
pub struct ModifyRollTask {
	modification: Option<RollModification>,
	modification_path: ModificationPath,
}
impl ModifyRollTask {
	pub fn new(modification: RollModification, path: ModificationPath) -> Self {
		Self {
			modification: Some(modification),
			modification_path: path,
		}
	}
}

impl PlayerTask for ModifyRollTask {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let modification = self
			.modification
			.take()
			.ok_or_else(|| SlayError::new("Cannot choose the same choice twice."))?;

		game
			.showdown
			.add_modification(self.modification_path, modification)?;
		let modification_task = game
			.showdown
			.get_modification_task(context, game, player_index);
		modification_task.apply(context, game);
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!(
			"Player modifying {:?} with {:?}",
			self.modification_path, self.modification
		)
	}
}

pub fn create_modify_roll_choice(
	context: &mut GameBookKeeping, _game: &Game, player_index: ids::PlayerIndex, card: &Card,
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
					DeckPath::Hand(player_index),
					card.id,
				)),
				arrows: vec![
					choices::DisplayArrow {
						source: choices::DisplayPath::CardIn(DeckPath::Hand(player_index), card.id),
						destination: choices::DisplayPath::DeckAt(DeckPath::Discard),
					},
					choices::DisplayArrow {
						source: DisplayPath::CardIn(DeckPath::Hand(player_index), card.id),
						destination: DisplayPath::Roll(*modification_path),
					},
				],
				label: format!(
					"Use {} to modify {}'s roll by {}",
					card.id, "somebody", modification_amount,
				),
				roll_modification_choice: Some(RollModificationChoice {
					choice_id,
					choice_type: RollModificationChoiceType::from_card(&card.spec, modification_amount),
				}),
			},
		),
		vec![
			Box::new(MoveCardTask {
				source: DeckPath::Hand(player_index),
				destination: DeckPath::Discard,
				card_id: card.id,
			}) as Box<dyn PlayerTask>,
			Box::new(ModifyRollTask::new(
				RollModification {
					modifying_player_index: player_index,
					card_id: card.id,
					modification_amount,
				},
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
//         context: &mut GameBookKeeping,
//         game: &mut Game,
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
	persist: RollCompletion,
}

impl SetCompleteTask {
	pub fn new(persist: RollCompletion) -> Self {
		Self { persist }
	}
}

impl PlayerTask for SetCompleteTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game
			.showdown
			.set_player_completion(player_index, self.persist)?;
		game.players[player_index].choices = None;
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("Setting completion to {:?}", self.persist)
	}
}

fn create_set_complete_choice(
	locator: ChoiceLocator, persist: RollCompletion, label: String,
) -> TasksChoice {
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
		vec![Box::new(SetCompleteTask::new(persist)) as Box<dyn PlayerTask>],
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
				source: DeckPath::Hand(player_index),
				destination: DeckPath::Discard,
				card_id: challenge_card_id,
			}) as Box<dyn PlayerTask>,
			Box::new(ChallengeTask::new()) as Box<dyn PlayerTask>,
		],
	)
}

#[derive(Debug, Clone)]
struct ChallengeTask {}
impl ChallengeTask {
	pub fn new() -> Self {
		Self {}
	}
}
impl PlayerTask for ChallengeTask {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game,
		challenging_player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let offer = game.showdown.take_current_offer()?;
		let mut challenge = offer.to_challenge(&mut context.rng, challenging_player_index)?;
		challenge.completion_tracker = Some(CompletionTracker::new(
			game.number_of_players(),
			deadlines::get_challenge_deadline(),
		));
		challenge.assign_all_choices(context, game);
		game.showdown.challenge(challenge);
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("Player is challenging.")
	}
}
