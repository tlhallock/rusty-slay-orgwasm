use crate::slay::choices::CardPath;
use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::showdown::completion::CompletionTracker;
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::showdown::roll_modification::ModificationOrigin;
use crate::slay::showdown::roll_modification::ModificationPath;
use crate::slay::showdown::roll_modification::RollModification;
use crate::slay::showdown::roll_modification::RollModificationChoiceType;
use crate::slay::specs::modifier::ModifierKinds;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::tasks::modify_roll::ModifyRollTask;
use crate::slay::tasks::tasks::move_card::MoveCardTask;

pub fn create_modify_roll_choice(
	context: &mut GameBookKeeping,
	player_index: ids::PlayerIndex,
	card_id: ids::CardId,
	modifier_kind: &ModifierKinds,
	modification_amount: i32,
	modification_path: &ModificationPath,
) -> TasksChoice {
	let choice_id = context.id_generator.generate();
	let _display_path =
		DisplayPath::CardAt(CardPath::TopCardIn(DeckPath::Hand(player_index), card_id));
	TasksChoice::new(
		choice_id,
		Choice::Modify(*modification_path, *modifier_kind, modification_amount),
		ChoiceDisplayType::Modify(RollModificationChoiceType::from_card(
			modifier_kind,
			modification_amount,
			*modification_path,
		)),
		vec![
			Box::new(MoveCardTask {
				source: DeckPath::Hand(player_index),
				destination: DeckPath::Discard,
				card_id,
			}) as Box<dyn PlayerTask>,
			Box::new(ModifyRollTask::new(
				RollModification {
					origin: ModificationOrigin::FromPlayer(player_index, *modifier_kind),
					amount: modification_amount,
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

pub fn create_challenge_choice(
	player_index: ids::PlayerIndex,
	id: ids::ChoiceId,
	challenge_card: &Card,
) -> TasksChoice {
	TasksChoice::new(
		id,
		Choice::Challenge,
		ChoiceDisplayType::Challenge(challenge_card.card_type),
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
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		challenging_player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let offer = game.showdown.take_current_offer()?;
		let mut challenge = offer.to_challenge(&mut context.rng, game, challenging_player_index)?;
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
