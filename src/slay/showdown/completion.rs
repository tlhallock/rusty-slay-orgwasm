use chrono::{DateTime, Utc};
use std::collections::HashMap;

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
	// #[serde(with = "ts_milliseconds_option")]
	pub deadline: Option<DateTime<Utc>>,
}

impl CompletionTracker {
	pub fn new(num_players: usize, deadline: Option<DateTime<Utc>>) -> Self {
		Self {
			deadline,
			player_completions: (0..num_players)
				.map(|index| (index, RollCompletion::Thinking))
				.collect(),
		}
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
}
