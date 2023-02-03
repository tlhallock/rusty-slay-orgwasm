use std::rc::Rc;
use std::vec;

use crate::slay::choices::ChoicePerspective;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesPerspective;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::modifier_visitors;
use crate::slay::showdown::completion::CompletionTracker;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::showdown::roll::Roll;
use crate::slay::showdown::roll_choices;
use crate::slay::showdown::roll_modification::ModificationPath;
use crate::slay::showdown::roll_modification::RollModification;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::state::game::Game;
use crate::slay::state::game::GameStaticInformation;
use crate::slay::tasks::tasks::set_complete;

// Only the party needs stacks...

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RollReason {
	UseHeroAbility(SlayCardSpec),
	AttackMonster(SlayCardSpec),
	Challenged,
	Challenging,
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

#[derive(Debug, PartialEq, Clone)]
pub struct RollPerspective {
	pub roller_index: ids::PlayerIndex,
	pub reason: RollReason,
	pub win_condition: Condition,
	pub loss_condition: Option<Condition>,
	pub initial: Roll,
	pub history: Vec<RollModification>,
	pub completion_tracker: CompletionTracker,
}

impl RollState {
	pub fn create(
		context: &mut GameBookKeeping,
		game: &Game,
		roller_index: ids::PlayerIndex,
		consequences: RollConsequences,
		// We could use this to create the tasks after the roll is done,
		// rather than creating the tasks ahead of time.
		reason: RollReason,
	) -> Self {
		Self {
			roller_index,
			initial: Roll::create_from(&mut context.rng),
			history: modifier_visitors::create_roll_history(game, roller_index, reason),
			consequences,
			completion_tracker: None,
			reason,
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
			+ self.history.iter().map(|h| h.amount).sum::<i32>()
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
			choices_type: ChoicesType::ModifyRoll,
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
		set_complete::create_set_completion_done(default_choice),
		set_complete::create_set_completion_until_modification(context.id_generator.generate()),
	];

	for card in game.players[player_index].hand.tops() {
		if let SlayCardSpec::ModifierCard(kind) = card.card_type {
			for modification_path in rolls.iter() {
				for modification_amount in kind.list_amounts() {
					choices.push(roll_choices::create_modify_roll_choice(
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

impl RollState {
	pub fn to_perspective(&self) -> RollPerspective {
		RollPerspective {
			roller_index: self.roller_index,
			reason: self.reason,
			win_condition: self.consequences.success.condition.to_owned(),
			loss_condition: self
				.consequences
				.loss
				.as_ref()
				.map(|c| c.condition.to_owned()),
			initial: self.initial.to_owned(),
			history: self.history.to_vec(),
			completion_tracker: self.completion_tracker.as_ref().unwrap().to_owned(),
		}
	}
}

impl RollPerspective {
	pub fn roller_name<'a>(&self, statics: &'a Rc<GameStaticInformation>) -> &'a String {
		&statics.players[self.roller_index].name
		// statics.player_name(self.roller_index)
	}
	pub fn won(&self) -> bool {
		self.win_condition.applies_to(self.calculate_roll_total())
	}
	pub fn lossed(&self) -> Option<bool> {
		self
			.loss_condition
			.as_ref()
			.map(|loss| loss.applies_to(self.calculate_roll_total()))
	}
	pub fn calculate_roll_total(&self) -> i32 {
		self.initial.calculate_total(&self.history)
	}
	pub fn choices(&self, choices: &Option<ChoicesPerspective>) -> Vec<ChoicePerspective> {
		if let Some(choices) = choices {
			choices
				.options
				.iter()
				.filter(|choice| choice.display.belongs_to_roll())
				.map(|choice| choice.to_owned())
				.collect()
		} else {
			Vec::new()
		}
	}
}
