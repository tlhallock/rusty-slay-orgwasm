use std::vec;

use crate::slay::choices::{ChoicePerspective, Choices, TasksChoice};
use crate::slay::deadlines::{self, Timeline};
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::showdown::common::Roll;
use crate::slay::showdown::common::RollModification;
use crate::slay::showdown::common::{ModificationPath, ModificationPerspective};
use crate::slay::showdown::completion::{CompletionTracker, PlayerCompletionPerspective};
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::showdown::roll_choices::{self, create_modify_roll_choice};
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::state::game::Game;

use super::consequences::Condition;

// Only the party needs stacks...

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RollReason {
	UseHeroAbility(SlayCardSpec),
	AttackMonster(SlayCardSpec),
}

#[derive(Debug, Clone)]
pub struct RollState {
	pub roller_index: ids::PlayerIndex,
	pub reason: RollReason,
	consequences: RollConsequences,
	pub initial: Roll,
	pub history: Vec<RollModification>,
	pub completion_tracker: Option<CompletionTracker>,
	foo: String,
}

impl RollState {
	pub fn create_roll_history(
		game: &Game,
		player_index: ids::PlayerIndex,
		reason: RollReason,
	) -> Vec<RollModification> {
		let mut ret = Vec::new();
		game.players[player_index].collect_roll_buffs(reason, &mut ret);
		ret
	}

	pub fn create(
		context: &mut GameBookKeeping,
		game: &Game,
		roller_index: ids::PlayerIndex,
		consequences: RollConsequences,
		reason: RollReason,
	) -> Self {
		Self {
			roller_index,
			initial: Roll::create_from(&mut context.rng),
			history: RollState::create_roll_history(game, roller_index, reason),
			consequences,
			completion_tracker: None,
			reason,
			foo: String::from("foo"),
		}
	}

	// pub fn new(
	// 	roller_index: ids::PlayerIndex,
	// 	consequences: RollConsequences,
	// 	initial: Roll,
	// 	reason: RollReason,
	// ) -> Self {
	// 	Self {
	// 		roller_index,
	// 		initial,
	// 		history: Default::default(),
	// 		consequences,
	// 		completion_tracker: None,
	// 		reason,
	// 	}
	// }

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
		self
			.consequences
			.apply_roll_sum(game, roll_sum, self.roller_index);
		// game.players[roll.roller_index].tasks = Some(roll.consequences.take_tasks(roll_sum));
	}
}

pub fn list_modification_choices(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	default_choice: ids::ChoiceId,
	rolls: Vec<ModificationPath>,
) -> Vec<TasksChoice> {
	let mut choices: Vec<TasksChoice> = vec![
		roll_choices::create_set_completion_done(default_choice),
		roll_choices::create_set_completion_until_modification(context.id_generator.generate()),
	];

	for card in game.players[player_index].hand.tops() {
		if let SlayCardSpec::ModifierCard(kind) = card.card_type {
			for modification_path in rolls.iter() {
				for modification_amount in kind.list_amounts() {
					choices.push(create_modify_roll_choice(
						context,
						player_index,
						card.id,
						&kind,
						modification_amount,
						modification_path,
					));
				}
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
	pub roller_name: String,
	pub initial: Roll,
	pub history: Vec<ModificationPerspective>,
	pub completions: Vec<PlayerCompletionPerspective>,
	pub timeline: Timeline,
	pub reason: RollReason,
	pub choices: Vec<ChoicePerspective>,

	pub roll_total: i32,

	pub win_condition: Condition,
	pub won: bool,
	pub loss_condition: Option<Condition>,
	pub lossed: Option<bool>,
}

impl RollState {
	pub fn to_perspective(&self, game: &Game, choices: &Option<&Choices>) -> RollPerspective {
		let roll_total = self.calculate_roll_total();
		RollPerspective {
			roller_name: game.get_player_name(self.roller_index),
			initial: self.initial.to_owned(),
			history: self
				.history
				.iter()
				.map(|m| m.to_perspective(game))
				.collect(),
			completions: self.tracker().to_perspective(game),

			timeline: self.tracker().timeline.to_owned(),
			reason: self.reason.to_owned(),
			choices: if let Some(choices) = choices {
				choices.choice_perspetives()
			} else {
				Vec::new()
			},

			roll_total,
			win_condition: self.consequences.success.condition.to_owned(),
			won: self.consequences.success.condition.applies_to(roll_total),
			loss_condition: self
				.consequences
				.loss
				.as_ref()
				.map(|rc| rc.condition.to_owned()),
			lossed: self
				.consequences
				.loss
				.as_ref()
				.map(|rc| rc.condition.applies_to(roll_total)),
		}
	}
}
