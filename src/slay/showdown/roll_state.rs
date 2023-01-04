use crate::slay::choices::{ChoiceLocator, Choices, TasksChoice};
use crate::slay::deadlines;

use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specification::CardSpec;
use crate::slay::state::Game;

use super::current_showdown::ShowDown;

use super::common::ModificationPath;
use super::common::Roll;
use super::common::RollModification;
use super::completion::CompletionTracker;
use super::consequences::RollConsequences;
use super::roll_choices::{self, create_modify_roll_choice};

// Only the party needs stacks...

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum RollReason {
	UseHeroAbility(CardSpec),
	AttackMonster(CardSpec),
}

#[derive(Debug, Clone)]
pub struct RollState {
	pub roller_index: usize,
	pub reason: RollReason,
	consequences: RollConsequences,
	pub initial: Roll,
	pub history: Vec<RollModification>,
	pub completion_tracker: Option<CompletionTracker>,
}

impl RollState {
	pub fn new(
		roller_index: usize, consequences: RollConsequences, initial: Roll, reason: RollReason,
	) -> Self {
		Self {
			roller_index,
			initial,
			history: Default::default(),
			consequences,
			completion_tracker: None,
			reason,
		}
	}

	pub fn add_modification(&mut self, modification: RollModification) {
		self.tracker_mut().timeline = deadlines::get_roll_deadline();
		self.history.push(modification);
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
	// fn set_player_completion(&mut self, player_index: usize, persist: RollCompletion) {
	//     self.completion_tracker.set_player_completion(player_index, persist);
	// }

	// fn should_offer_modifications_again(&self, player_index: usize) -> bool {
	//     self.completion_tracker.should_offer_modifications_again(player_index)
	// }
}

impl ShowDown for RollState {
	fn tracker(&self) -> &CompletionTracker {
		&self.completion_tracker.as_ref().unwrap()
	}

	fn tracker_mut(&mut self) -> &mut CompletionTracker {
		self.completion_tracker.as_mut().unwrap()
	}

	fn create_choice_for(
		&self, context: &mut GameBookKeeping, game: &Game, player_index: usize,
	) -> Choices {
		let default_choice = context.id_generator.generate();
		Choices {
			instructions: "Choose whether to modify the current roll.".to_string(),
			default_choice,
			timeline: self.tracker().timeline.to_owned(),
			options: list_modification_choices(
				context,
				game,
				player_index,
				default_choice,
				vec![ModificationPath::Roll],
			),
		}
	}

	fn finish(&mut self, _context: &mut GameBookKeeping, game: &mut Game) {
		let roll_sum = self.calculate_roll_total();
		self.consequences.apply_roll_sum(game, roll_sum);
		// game.players[roll.roller_index].tasks = Some(roll.consequences.take_tasks(roll_sum));
	}
}

pub fn list_modification_choices(
	context: &mut GameBookKeeping, game: &Game, player_index: usize, default_choice: ids::ChoiceId,
	rolls: Vec<ModificationPath>,
) -> Vec<TasksChoice> {
	let mut choices: Vec<TasksChoice> = vec![
		roll_choices::create_set_completion_done(ChoiceLocator {
			id: default_choice,
			player_index,
		}),
		roll_choices::create_set_completion_until_modification(ChoiceLocator {
			id: context.id_generator.generate(),
			player_index,
		}),
	];

	for stack in game.players[player_index].hand.iter() {
		let card = &stack.top;
		for modification_amount in card.spec.modifiers.iter() {
			for modification_path in rolls.iter() {
				choices.push(create_modify_roll_choice(
					context,
					game,
					player_index,
					card,
					*modification_amount,
					modification_path,
				))
			}
		}
	}
	// for modification_amount in

	//   choices.extend(
	//       player
	//           .hand
	//           .stacks
	//           .iter()
	//           .flat_map(|s| s.top.modification_amounts())
	//           .flat_map(|(card_id, modification_amount)| {
	//               rolls
	//                   .iter()
	//                   .map(|path| {
	//                       (
	//                           card_id,
	//                           modification_amount,
	//                           path,
	//                       )
	//                   })
	//                   .collect::<Vec<(ids::CardId, i32, ids::RollId, choices::DisplayPath)>>()
	//           })
	//           .map(|(card_id, modification_amount, roll_id, display_path)| {

	//           }),
	//   );
	choices
}

// pub fn do_roll(
//     context: &mut game_context::GameBookKeeping,
//     game: &mut state::Game,
//     roller_index: usize,
//     consequences: RollConsequences,
// ) {
//     let roll = RollState::new(
//         roller_index,
//         consequences,
//         Roll::create_from(&mut context.rng),
//         game.number_of_players(),
//     );
//     for player in game.players.iter_mut() {
//         assign_roll_choices(context, player, &roll);
//     }
//     // game.players
//     //     .iter_mut()
//     //     .for_each(|player| assign_roll_choices(context, player, &roll));
//     game.roll = Some(roll);
// }
