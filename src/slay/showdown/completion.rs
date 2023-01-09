use std::collections::HashMap;

use crate::slay::deadlines::Timeline;
use crate::slay::ids;
use crate::slay::state::game::Game;

// #[derive(Debug, Clone, PartialEq, Eq, Copy)]
// pub enum CompletionPath {
//     Roll,
//     OfferChallenges,
//     Challege,
// }

// Rename this to ShowdownCompletion
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Completion {
	Thinking,
	DoneUntilModification,
	AllDone,
}

impl Completion {
	pub fn done(&self) -> bool {
		match self {
			Self::Thinking => false,
			Self::DoneUntilModification => true,
			Self::AllDone => true,
		}
	}
	pub fn offer_on_modify(&self) -> bool {
		match self {
			Self::Thinking => true,
			Self::DoneUntilModification => true,
			Self::AllDone => false,
		}
	}
}

#[derive(Debug, Clone)]
pub struct CompletionTracker {
	pub player_completions: HashMap<ids::PlayerIndex, Completion>,
	pub timeline: Timeline,
}

impl CompletionTracker {
	pub fn new(num_players: ids::PlayerIndex, timeline: Timeline) -> Self {
		Self {
			timeline,
			player_completions: (0..num_players)
				.map(|index| (index, Completion::Thinking))
				.collect(),
		}
	}

	pub fn update_deadline(&mut self, new_timeline: Timeline) {
		self.timeline = new_timeline;
	}

	pub fn is_complete(&self) -> bool {
		// Check the deadline!?
		let all_done = self.player_completions.values().all(|rc| rc.done());
		log::info!("showdown completion: {}", all_done);
		all_done
	}

	pub fn set_player_completion(&mut self, player_index: ids::PlayerIndex, completion: Completion) {
		self.player_completions.insert(player_index, completion);
	}

	pub fn should_offer_modifications_again(&self, player_index: ids::PlayerIndex) -> bool {
		self
			.player_completions
			.get(&player_index)
			.unwrap()
			.offer_on_modify()
	}

	pub(crate) fn reset_timeline(&mut self) {
		self.timeline.reset();
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerCompletionPerspective {
	pub player_name: String,
	pub completion: Completion,
}

impl CompletionTracker {
	pub fn to_perspective(&self, game: &Game) -> Vec<PlayerCompletionPerspective> {
		self
			.player_completions
			.iter()
			.map(|(player_index, completion)| PlayerCompletionPerspective {
				player_name: game.players[*player_index].name.to_owned(),
				completion: *completion,
			})
			.collect()
	}
}
