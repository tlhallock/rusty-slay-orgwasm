use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::slay::deadlines::{self, Timeline};

// #[derive(Debug, Clone, PartialEq, Eq, Copy)]
// pub enum CompletionPath {
//     Roll,
//     OfferChallenges,
//     Challege,
// }

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum RollCompletion {
	Thinking,
	DoneUntilModification,
	AllDone,
}

impl RollCompletion {
	pub fn done(&self) -> bool {
		match self {
			Self::Thinking => false,
			Self::DoneUntilModification => true,
			Self::AllDone => true,
		}
	}
}

#[derive(Debug, Clone)]
pub struct CompletionTracker {
	pub player_completions: HashMap<usize, RollCompletion>,
	pub timeline: Timeline,
}

impl CompletionTracker {
	pub fn new(num_players: usize, timeline: Timeline) -> Self {
		Self {
			timeline,
			player_completions: (0..num_players)
				.map(|index| (index, RollCompletion::Thinking))
				.collect(),
		}
	}

	pub fn update_deadline(&mut self, new_timeline: Timeline) {
		self.timeline = new_timeline;
	}

	pub fn is_complete(&self) -> bool {
		// Check the deadline!?
		self.player_completions.values().all(|rc| rc.done())
	}

	pub fn set_player_completion(&mut self, player_index: usize, completion: RollCompletion) {
		self.player_completions.insert(player_index, completion);
	}

	pub fn should_offer_modifications_again(&self, player_index: usize) -> bool {
		let result = self.player_completions.get(&player_index).unwrap();
		*result == RollCompletion::DoneUntilModification
		// self.player_completions.get(&player_index).as_deref().contains(&RollCompletion::DoneUntilModification)
	}

	pub(crate) fn reset_timeline(&mut self) {
		self.timeline.reset();
	}
}
