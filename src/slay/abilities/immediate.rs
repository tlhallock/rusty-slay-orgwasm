use std::collections::HashSet;

use crate::slay::actions;
use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specification::CardType;
use crate::slay::state::game::Game;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskParamName;
use crate::slay::tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct OfferPlayImmediately {
	card_param: TaskParamName,
	card_types: Option<HashSet<CardType>>,
}

impl OfferPlayImmediately {
	pub fn create(
		card_param: TaskParamName, card_types: Option<HashSet<CardType>>,
	) -> Box<dyn PlayerTask> {
		Box::new(OfferPlayImmediately {
			card_param,
			card_types,
		})
	}

	fn does_not_apply(&self, card: &crate::slay::state::stack::Card) -> bool {
		if let Some(card_types) = self.card_types.as_ref() {
			card_types.contains(&card.card_type())
		} else {
			true
		}
	}
}

impl PlayerTask for OfferPlayImmediately {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		/////////////////
		let card_id = game.card_param(player_index, &self.card_param)?;
		if card_id.is_none() {
			return Ok(TaskProgressResult::TaskComplete);
		}

		let card_id = card_id.unwrap();
		let card = game
			// .deck(deck_path)
			.find_card(card_id)
			.ok_or_else(|| SlayError::new("Unable to find card."))?;

		if self.does_not_apply(card) {
			return Ok(TaskProgressResult::TaskComplete);
		}

		let mut play_immediately_tasks = Vec::new();
		if let Some(item_modifier) = card.spec.item_modifier.as_ref() {
			// src/slay/actions.rs:452
			let players_with_stacks = game.players_with_stacks();
			if players_with_stacks.is_empty() {
				log::info!("There are no places to put the item.");
				return Ok(TaskProgressResult::TaskComplete);
			}
			play_immediately_tasks.push(actions::create_place_item_challenge_offer(
				player_index,
				card,
				item_modifier,
				players_with_stacks,
			));
		}

		let default_choice = context.id_generator.generate();
		game.players[player_index].choices = Some(Choices {
			instructions: format!("You have received {}, would you like to play it immediately?", card.label()),
			timeline: deadlines::get_refactor_me_deadline(),
			default_choice: Some(default_choice),
			options: vec![
				TasksChoice::prepend(
					context.id_generator.generate(),
					ChoiceDisplay {
						display_type: ChoiceDisplayType::Yes,
						label: "Yes".to_string(),
					},
					play_immediately_tasks,
				),
				TasksChoice::prepend(
					default_choice,
					ChoiceDisplay {
						display_type: ChoiceDisplayType::No,
						label: "No".to_string(),
					},
					vec![],
				),
			],
		});

		/////////////////
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		"Offer to play a card immediately".to_owned()
	}
}
