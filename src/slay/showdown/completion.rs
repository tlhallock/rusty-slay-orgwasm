use crate::slay::deadlines::Timeline;
use crate::slay::ids;

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

#[derive(Debug, Clone, PartialEq)]
pub struct CompletionTracker {
	pub completions: Vec<Completion>,
	pub timeline: Timeline,
}

impl CompletionTracker {
	pub fn new(num_players: ids::PlayerIndex, timeline: Timeline) -> Self {
		Self {
			timeline,
			completions: vec![Completion::Thinking; num_players],
		}
	}

	pub fn update_deadline(&mut self, new_timeline: Timeline) {
		self.timeline = new_timeline;
	}

	pub fn is_complete(&self) -> bool {
		let ret = self.completions.iter().all(|rc| rc.done()) || self.timeline.is_complete();
		// Check the deadline!?
		log::info!("showdown completion: {}", ret);
		ret
	}

	pub fn set_player_completion(&mut self, player_index: ids::PlayerIndex, completion: Completion) {
		self.completions[player_index] = completion;
	}

	pub fn should_offer_modifications_again(&self, player_index: ids::PlayerIndex) -> bool {
		self.completions[player_index].offer_on_modify()
	}

	pub(crate) fn reset_timeline(&mut self) {
		self.timeline.reset();
	}
}

// #[derive(Debug, PartialEq, Clone)]
// pub struct PlayerCompletionPerspective {
// 	pub player_name: String,
// 	pub completion: Completion,
// }

// impl CompletionTracker {
// 	pub fn to_perspective(&self, game: &Game) -> Vec<PlayerCompletionPerspective> {
// 		self
// 			.player_completions
// 			.iter()
// 			.map(|(player_index, completion)| PlayerCompletionPerspective {
// 				player_name: game.players[*player_index].name.to_owned(),
// 				completion: *completion,
// 			})
// 			.collect()
// 	}
// }
