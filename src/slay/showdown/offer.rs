
use crate::slay::deadlines::Timeline;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::choices::{ChoicePerspective, Choices, TasksChoice};
use crate::slay::errors::SlayResult;
use crate::slay::specification::CardType;
use crate::slay::showdown::challenge::ChallengeState;
use crate::slay::showdown::completion::CompletionTracker;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::state::game::Game;
use crate::slay::showdown::common::ChallengeReason;
use crate::slay::showdown::completion::PlayerCompletionPerspective;
use crate::slay::showdown::roll_choices::{self, create_challenge_choice};

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
		let mut ret = vec![roll_choices::create_set_completion_done(default_choice)];

		for card in game.players[challenging_player_index].hand.tops() {
			if card.card_type() != &CardType::Challenge {
				continue;
			}
			ret.push(create_challenge_choice(
				challenging_player_index,
				context.id_generator.generate(),
				card,
			));
			break;
		}
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
		&self, game: &Game, choices: &Option<&Choices>,
	) -> OfferChallengesPerspective {
		OfferChallengesPerspective {
			initiator: game.players[self.player_index].name.to_owned(),
			completions: self.tracker().to_perspective(game),
			reason: self.reason.to_owned(),
			choices: if let Some(choices) = choices {
				choices.choice_perspetives()
			} else {
				Vec::new()
			},
			timeline: self.tracker().timeline.to_owned(),
		}
	}
}
