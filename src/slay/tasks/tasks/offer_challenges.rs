use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;

use crate::slay::showdown::completion::Completion;
use crate::slay::showdown::completion::CompletionTracker;

use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::showdown::offer::OfferChallengesState;

use crate::slay::state::game::Game;

use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct OfferChallengesTask {
	offer: Option<OfferChallengesState>,
}

impl OfferChallengesTask {
	pub fn new(offer: OfferChallengesState) -> Self {
		Self { offer: Some(offer) }
	}
}
impl PlayerTask for OfferChallengesTask {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		_player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		log::info!("making progress");
		if let Some(mut offer) = self.offer.take() {
			let mut completion_tracker = CompletionTracker::new(
				game.number_of_players(),
				deadlines::get_offer_challenges_deadline(),
			);
			// The current player is not allowed to challenge himself...
			completion_tracker.set_player_completion(offer.player_index, Completion::AllDone);
			offer.completion_tracker = Some(completion_tracker);
			offer.assign_all_choices(context, game);
			game.showdown.offer(offer);
			log::info!("set the offer...");
			Ok(TaskProgressResult::TaskComplete)
		} else {
			Err(SlayError::new("Can only perform a choice once..."))
		}
	}
	fn label(&self) -> String {
		format!("Offering challenges for {:?}", self.offer)
	}
}
