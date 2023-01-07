use crate::slay::deadlines::Timeline;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;

use crate::slay::choices::{
	ChoiceLocator, ChoicePerspective, Choices, ChoicesPerspective, TasksChoice,
};
use crate::slay::errors::SlayResult;
use crate::slay::specification::CardType;

use crate::slay::showdown::challenge::ChallengeState;
use crate::slay::showdown::completion::CompletionTracker;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::state::game::Game;

use super::common::ChallengeReason;
use super::completion::PlayerCompletionPerspective;
use super::roll_choices::{self, create_challenge_choice};

#[derive(Debug, Clone)]
pub struct OfferChallengesState {
	pub player_index: ids::PlayerIndex,
	pub reason: ChallengeReason,
	pub completion_tracker: Option<CompletionTracker>,
	consequences: RollConsequences,
}

impl OfferChallengesState {
	pub fn new(
		player_index: ids::PlayerIndex, consequences: RollConsequences, reason: ChallengeReason,
	) -> Self {
		Self {
			player_index,
			completion_tracker: None,
			consequences,
			reason,
		}
	}

	fn list_challenge_choices(
		&self,
		context: &mut GameBookKeeping,
		game: &Game,
		challenging_player_index: ids::PlayerIndex,
		// challenging_player: &Player,
		default_choice: u32,
	) -> Vec<TasksChoice> {
		let mut ret = vec![roll_choices::create_set_completion_done(ChoiceLocator {
			id: default_choice,
			player_index: challenging_player_index,
		})];
		ret.extend(
			game.players[challenging_player_index]
				.hand
				.list_top_cards_by_type(&CardType::Challenge)
				.first()
				.iter()
				.map(|card_id| {
					create_challenge_choice(
						ChoiceLocator {
							id: context.id_generator.generate(),
							player_index: challenging_player_index,
						},
						**card_id,
					)
				}),
		);
		ret
	}

	pub fn to_challenge(
		&self, rng: &mut rand::rngs::ThreadRng, challenger_index: ids::PlayerIndex,
	) -> SlayResult<ChallengeState> {
		Ok(ChallengeState::new(
			rng,
			self.player_index,
			challenger_index,
			self.consequences.to_owned(), // Copied, although it is about to be dropped.
			self.reason.to_owned(),
		))
	}
}

impl ShowDown for OfferChallengesState {
	fn tracker(&self) -> &CompletionTracker {
		self.completion_tracker.as_ref().unwrap()
	}

	fn tracker_mut(&mut self) -> &mut CompletionTracker {
		self.completion_tracker.as_mut().unwrap()
	}

	fn create_choice_for(
		&self, context: &mut GameBookKeeping, game: &Game, player_index: ids::PlayerIndex,
	) -> Choices {
		let default_choice = context.id_generator.generate();
		Choices {
			instructions: "Choose whether to modify the current challenge.".to_string(),
			default_choice: Some(default_choice),
			timeline: self.tracker().timeline.to_owned(),
			options: self.list_challenge_choices(context, game, player_index, default_choice),
		}
	}

	fn finish(&mut self, context: &mut GameBookKeeping, game: &mut Game) {
		self.consequences.proceed(context, game, self.player_index);
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct OfferChallengesPerspective {
	pub initiator: String,
	pub completions: Vec<PlayerCompletionPerspective>,
	pub reason: ChallengeReason,
	pub choices: Vec<ChoicePerspective>,
	pub timeline: Timeline,
}

impl OfferChallengesState {
	pub fn to_perspective(
		&self, game: &Game, choices: &Option<ChoicesPerspective>,
	) -> OfferChallengesPerspective {
		OfferChallengesPerspective {
			initiator: game.players[self.player_index].name.to_owned(),
			completions: self.tracker().to_perspective(game),
			reason: self.reason.to_owned(),
			choices: choices
				.iter()
				.flat_map(|choices| choices.actions.clone())
				.collect(),
			timeline: self.tracker().timeline.to_owned(),
		}
	}
}
