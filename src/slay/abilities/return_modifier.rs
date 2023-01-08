




use std::collections::HashSet;

use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceInformation;
use crate::slay::choices::ChoiceLocator;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::MoveCardTask;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskProgressResult;

// #[derive(Debug, Clone)]
// pub struct ReturnModifier {
// 	num: u32,
// 	include: Option<HashSet<ids::CardId>>,
// }

// impl Discard {
// 	pub fn new(num: u32) -> Self {
// 		Self { num, include: None }
// 	}
// 	pub fn discard_one_of(include: HashSet<ids::CardId>) -> Self {
// 		Self {
// 			num: 1,
// 			include: Some(include),
// 		}
// 	}

// 	pub fn should_include(&self, card_id: ids::CardId) -> bool {
// 		if let Some(include) = self.include {
// 			include.contains(&card_id)
// 		} else {
// 			true
// 		}
// 	}
// }

// impl PlayerTask for Discard {
// 	fn make_progress(
// 		&mut self, context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
// 	) -> SlayResult<TaskProgressResult> {
// 		if self.num == 0 {
// 			return Ok(TaskProgressResult::TaskComplete);
// 		}
// 		// check if the number to discard is the same as the number of cards in your hand.
// 		self.num -= 1;
// 		let options: Vec<TasksChoice> = game.players[player_index]
// 			.hand
// 			.iter()
// 			.filter(|stack| self.should_include(stack.top.id))
// 			.map(|stack| {
// 				TasksChoice::prepend(
// 					ChoiceInformation {
// 						locator: ChoiceLocator {
// 							id: context.id_generator.generate(),
// 							player_index,
// 						},
// 						display: ChoiceDisplay {
// 							highlight: Some(DisplayPath::CardIn(
// 								DeckPath::Hand(player_index),
// 								stack.top.id,
// 							)),
// 							arrows: vec![], // Todo
// 							label: format!("Discard {}", stack.top.spec.label),
// 							roll_modification_choice: None,
// 						},
// 					},
// 					vec![Box::new(MoveCardTask {
// 						source: DeckPath::Hand(player_index),
// 						destination: DeckPath::Discard,
// 						card_id: stack.top.id,
// 					}) as Box<dyn PlayerTask>],
// 				)
// 			})
// 			.collect();

// 		if options.is_empty() {
// 			return Ok(TaskProgressResult::TaskComplete);
// 		}
// 		let default_choice = options[0].get_choice_information().locator.id;

// 		game.players[player_index].choices = Some(Choices::new(
// 			options,
// 			Some(default_choice),
// 			deadlines::get_discard_deadline(),
// 			"Choose a card to discard.".to_owned(),
// 		));
// 		Ok(TaskProgressResult::ProgressMade)
// 	}
// 	fn label(&self) -> String {
// 		format!("Player must discard {} cards", self.num)
// 	}
// }
