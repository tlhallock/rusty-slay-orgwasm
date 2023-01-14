use super::{state::{game::Game, summarizable::Summarizable}, ids, choices::{Choice, ChoicesType}};








#[derive(Clone, Debug)]
pub enum Notification {
	PlayerChose(ids::PlayerIndex, Choice),
	PlayerIsChoosing(ids::PlayerIndex, ChoicesType),
	Modification,
	InitialRoll(i32),
	RollResult(bool), // show the roll value and threshold?
	OfferResult, // Option<ids::PlayerIndex>
	ChallengeResult(bool),
	// Player Complete?
	PlayerWon(ids::PlayerIndex),
	PlayersTurn(ids::PlayerIndex),


}

impl Notification {
	pub fn get_description(&self, game: &Game, player_index: ids::PlayerIndex) -> String {
		match self {
    Notification::PlayerChose(choice) => todo!(),
}
	}
}

impl Summarizable for Notification {
  fn summarize<W: std::io::Write>(
		&self,
		f: &mut std::io::BufWriter<W>,
		indentation_level: u32,
	) -> Result<(), std::io::Error> {
      todo!()
  }
}


// impl Notification {
// 	pub fn new(message_text: String) -> Self {
// 		Notification { message_text }
// 	}
// }

// impl From<&'static str> for Notification {
// 	fn from(value: &'static str) -> Self {
// 		Notification {
// 			message_text: value.to_string(),
// 		}
// 	}
// }
