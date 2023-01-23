use crate::slay::choices::ChoicePerspective;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesPerspective;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::TasksChoice;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::showdown::challenge::ChallengeState;
use crate::slay::showdown::completion::CompletionTracker;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::showdown::roll::ChallengeReason;
use crate::slay::showdown::roll_choices;
use crate::slay::state::game::Game;
use crate::slay::tasks::tasks::set_complete;

#[derive(Debug, Clone)]
pub struct OfferChallengesState {
	pub player_index: ids::PlayerIndex,
	pub reason: ChallengeReason,
	pub completion_tracker: Option<CompletionTracker>,
	consequences: RollConsequences,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OfferChallengesPerspective {
	pub player_index: ids::PlayerIndex,
	pub reason: ChallengeReason,
	pub completion_tracker: CompletionTracker,
}

impl OfferChallengesPerspective {
	pub fn choices(&self, choices: &Option<ChoicesPerspective>) -> Vec<ChoicePerspective> {
		if let Some(choices) = choices {
			choices
				.options
				.iter()
				.filter(|choice| choice.display.belongs_to_offer())
				.map(|choice| choice.to_owned())
				.collect()
		} else {
			Vec::new()
		}
	}
}

impl OfferChallengesState {
	pub fn new(
		player_index: ids::PlayerIndex,
		consequences: RollConsequences,
		reason: ChallengeReason,
	) -> Self {
		Self {
			player_index,
			completion_tracker: None,
			consequences,
			reason,
		}
	}

	pub fn to_perspective(&self) -> OfferChallengesPerspective {
		OfferChallengesPerspective {
			player_index: self.player_index,
			completion_tracker: self.completion_tracker.as_ref().unwrap().to_owned(),
			reason: self.reason.to_owned(),
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
		let mut ret = vec![set_complete::create_set_completion_done(default_choice)];

		if let Some(card) = game.players[challenging_player_index]
			.hand
			.tops()
			.find(|card| card.is_challenge())
		{
			ret.push(roll_choices::create_challenge_choice(
				challenging_player_index,
				context.id_generator.generate(),
				card,
			))
		}
		ret
	}

	pub fn to_challenge(
		&self,
		rng: &mut rand::rngs::ThreadRng,
		game: &Game,
		challenger_index: ids::PlayerIndex,
	) -> SlayResult<ChallengeState> {
		Ok(ChallengeState::new(
			rng,
			game,
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
		&self,
		context: &mut GameBookKeeping,
		game: &Game,
		player_index: ids::PlayerIndex,
	) -> Choices {
		let default_choice = context.id_generator.generate();
		Choices {
			choices_type: ChoicesType::OfferChallenges,
			default_choice: Some(default_choice),
			timeline: self.tracker().timeline.to_owned(),
			options: self.list_challenge_choices(context, game, player_index, default_choice),
		}
	}

	fn finish(&mut self, context: &mut GameBookKeeping, game: &mut Game) {
		self.consequences.proceed(context, game, self.player_index);
	}
}
