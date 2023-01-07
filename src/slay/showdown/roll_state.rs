use crate::slay::choices::{ChoiceLocator, Choices, TasksChoice, ChoicePerspective, ChoicesPerspective};
use crate::slay::deadlines::{self, Timeline};

use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specification::CardSpec;
use crate::slay::state::game::Game;
use crate::slay::state::stack::CardSpecPerspective;

use super::current_showdown::ShowDown;

use super::common::{ModificationPath, ModificationPerspective};
use super::common::Roll;
use super::common::RollModification;
use super::completion::{CompletionTracker, PlayerCompletionPerspective};
use super::consequences::RollConsequences;
use super::roll_choices::{create_modify_roll_choice, self};

// Only the party needs stacks...

#[derive(Debug, PartialEq, Clone)]
pub enum RollReason {
	UseHeroAbility(CardSpecPerspective),
	AttackMonster(CardSpecPerspective),
}

#[derive(Debug, Clone)]
pub struct RollState {
	pub roller_index: ids::PlayerIndex,
	pub reason: RollReason,
	consequences: RollConsequences,
	pub initial: Roll,
	pub history: Vec<RollModification>,
	pub completion_tracker: Option<CompletionTracker>,
}

impl RollState {
	pub fn new(
		roller_index: ids::PlayerIndex, consequences: RollConsequences, initial: Roll,
		reason: RollReason,
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
}

impl ShowDown for RollState {
	fn tracker(&self) -> &CompletionTracker {
		&self.completion_tracker.as_ref().unwrap()
	}

	fn tracker_mut(&mut self) -> &mut CompletionTracker {
		self.completion_tracker.as_mut().unwrap()
	}

	fn create_choice_for(
		&self, context: &mut GameBookKeeping, game: &Game, player_index: ids::PlayerIndex,
	) -> Choices {
		let default_choice = context.id_generator.generate();
		Choices {
			instructions: "Choose whether to modify the current roll.".to_string(),
			default_choice: Some(default_choice),
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
		self.consequences.apply_roll_sum(game, roll_sum, self.roller_index);
		// game.players[roll.roller_index].tasks = Some(roll.consequences.take_tasks(roll_sum));
	}
}

pub fn list_modification_choices(
	context: &mut GameBookKeeping, game: &Game, player_index: ids::PlayerIndex,
	default_choice: ids::ChoiceId, rolls: Vec<ModificationPath>,
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




#[derive(Debug, PartialEq, Clone)]
pub struct RollPerspective {
	id: ids::RollId,
	pub roller_name: String,
	pub initial: Roll,
	pub history: Vec<ModificationPerspective>,
	pub completions: Vec<PlayerCompletionPerspective>,
	pub roll_total: i32,
	pub success: bool,
	pub timeline: Timeline,
	pub reason: RollReason,
	pub choices: Vec<ChoicePerspective>,
}

impl RollState {
	pub fn to_perspective(
		&self, game: &Game, choices: &Option<ChoicesPerspective>,
	) -> RollPerspective {
		RollPerspective {
			id: 0, // Need to fill this in again?
			roller_name: game.players[self.roller_index].name.to_owned(),
			initial: self.initial.to_owned(),
			history: self
				.history
				.iter()
				.map(|m| m.to_perspective(game))
				.collect(),
			completions: self.tracker().to_perspective(game),
			roll_total: self.calculate_roll_total(),
			success: false,
			timeline: self.tracker().timeline.to_owned(),
			reason: self.reason.to_owned(),
			choices: choices
				.iter()
				.map(|choices| {
					choices
						.actions
						.iter()
						.map(|o| o.to_owned())
						.collect::<Vec<ChoicePerspective>>()
				})
				.flatten()
				.collect(),
		}
	}
}
