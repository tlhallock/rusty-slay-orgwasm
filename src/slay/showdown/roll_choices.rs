use crate::slay::choices::CardPath;
use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::{SlayResult};
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::showdown::common::ModificationPath;
use crate::slay::showdown::common::RollModification;
use crate::slay::showdown::common::RollModificationChoiceType;
use crate::slay::showdown::completion::{Completion, CompletionTracker};
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks::{MoveCardTask, PlayerTask, TaskProgressResult};

#[derive(Debug, Clone)]
pub struct ModifyRollTask {
	modification: RollModification,
	modification_path: ModificationPath,
}
impl ModifyRollTask {
	pub fn new(modification: RollModification, path: ModificationPath) -> Self {
		Self {
			modification,
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
			.to_owned();

		game
			.showdown
			.add_modification(self.modification_path, modification)?;
		let modification_task = game
			.showdown
			.get_modification_task(context, game);
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
	let _display_path =
		DisplayPath::CardAt(CardPath::TopCardIn(DeckPath::Hand(player_index), card.id));
	TasksChoice::new(
		choice_id,
		ChoiceDisplay {
			display_type: ChoiceDisplayType::Modify(RollModificationChoiceType::from_card(
				&card.spec,
				modification_amount,
				*modification_path,
			)),
			label: format!(
				"Use {} to modify {}'s roll by {}",
				card.id, "somebody", modification_amount,
			),
		},
		vec![
			Box::new(MoveCardTask {
				source: DeckPath::Hand(player_index),
				destination: DeckPath::Discard,
				card_id: card.id,
			}) as Box<dyn PlayerTask>,
			Box::new(ModifyRollTask::new(
				RollModification {
					modifying_player_index: player_index,
					// card_path: CardPath::TopCardIn(DeckPath::Hand(player_index), card.id),
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
	persist: Completion,
}

impl SetCompleteTask {
	pub fn new(persist: Completion) -> Self {
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
	id: ids::ChoiceId, persist: Completion, label: String,
) -> TasksChoice {
	TasksChoice::new(
		id,
		ChoiceDisplay {
			label,
			display_type: ChoiceDisplayType::SetCompletion(persist),
		},
		vec![Box::new(SetCompleteTask::new(persist)) as Box<dyn PlayerTask>],
	)
}

pub fn create_set_completion_done(id: ids::ChoiceId) -> TasksChoice {
	create_set_complete_choice(id, Completion::AllDone, "Do nothing.".to_string())
}

pub fn create_set_completion_until_modification(id: ids::ChoiceId) -> TasksChoice {
	create_set_complete_choice(
		id,
		Completion::DoneUntilModification,
		"Don't modify this roll unless someone else does.".to_string(),
	)
}

pub fn create_challenge_choice(
	player_index: ids::PlayerIndex, id: ids::ChoiceId, challenge_card: &Card,
) -> TasksChoice {
	TasksChoice::new(
		id,
		ChoiceDisplay {
			display_type: ChoiceDisplayType::Challenge(challenge_card.as_perspective()),
			label: "Challenge!".to_string(),
		},
		vec![
			Box::new(MoveCardTask {
				source: DeckPath::Hand(player_index),
				destination: DeckPath::Discard,
				card_id: challenge_card.id,
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
		"Player is challenging.".to_string()
	}
}
