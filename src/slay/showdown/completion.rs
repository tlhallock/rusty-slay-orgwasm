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
	pub player_completions: HashMap<ids::PlayerIndex, RollCompletion>,
	pub timeline: Timeline,
}

impl CompletionTracker {
	pub fn new(num_players: ids::PlayerIndex, timeline: Timeline) -> Self {
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
		log::info!("Looking at the completions...");
		for v in self.player_completions.values() {
			log::info!("{:?} {:?}", v, v.done());
		}
		self.player_completions.values().all(|rc| rc.done())
	}

	pub fn set_player_completion(
		&mut self, player_index: ids::PlayerIndex, completion: RollCompletion,
	) {
		self.player_completions.insert(player_index, completion);
	}

	pub fn should_offer_modifications_again(&self, player_index: ids::PlayerIndex) -> bool {
		let result = self.player_completions.get(&player_index).unwrap();
		*result == RollCompletion::DoneUntilModification
		// self.player_completions.get(&player_index).as_deref().contains(&RollCompletion::DoneUntilModification)
	}

	pub(crate) fn reset_timeline(&mut self) {
		self.timeline.reset();
	}
}





#[derive(Debug, PartialEq, Clone)]
pub struct PlayerCompletionPerspective {
	pub player_name: String,
	pub completion: RollCompletion,
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