use crate::slay::choices::Choice;
use crate::slay::choices::ChoicesType;
use crate::slay::ids;
use crate::slay::state::game::GameStaticInformation;
use crate::slay::state::summarizable::Summarizable;

use super::specs::cards::SlayCardSpec;

#[derive(Clone, Debug, PartialEq)]
pub enum Notification {
	PlayerChose(ids::PlayerIndex, Choice),
	PlayerIsChoosing(ids::PlayerIndex, ChoicesType),
	Modification,
	InitialRoll(i32),
	RollResult(bool), // show the roll value and threshold?
	OfferResult,      // Option<ids::PlayerIndex>
	ChallengeResult(bool),
	// Player Complete?
	PlayerWon(ids::PlayerIndex),
	PlayersTurn(ids::PlayerIndex),

	NoWhereToPlaceItem,
	PlayerDrew(ids::PlayerIndex, SlayCardSpec),
}

impl Notification {
	pub fn get_description(
		&self,
		statics: &GameStaticInformation,
		viewer: ids::PlayerIndex,
	) -> String {
		match self {
			Notification::PlayerChose(viewed, choice) => format!(
				"{} chose {}",
				statics.players_name_from_perspective(viewer, *viewed),
				choice.get_notification(statics, *viewed),
			),
			Notification::PlayerIsChoosing(viewed, choices_type) => format!(
				"{} is choosing {:?}",
				statics.players_name_from_perspective(viewer, *viewed),
				choices_type,
			),
			Notification::Modification => format!("There has been a modification {}", "sir",),
			Notification::InitialRoll(roll_amount) => format!("Someone rolled a {}", roll_amount,),
			Notification::RollResult(success) => format!("The result of the roll was {}", success),
			Notification::OfferResult => String::from("Challenges are no longer accepted."),
			Notification::ChallengeResult(success) => {
				format!("The result of the challenge was {:?}", success,)
			}
			Notification::PlayerWon(viewed) => format!(
				"The game is over! {} won!",
				statics.players_name_from_perspective(viewer, *viewed),
			),
			Notification::PlayersTurn(viewed) => format!(
				"It is now {}'s turn.",
				statics.players_name_from_perspective(viewer, *viewed),
			),
			Notification::NoWhereToPlaceItem => String::from("There was no where to place an item card."),
			Notification::PlayerDrew(player_index, spec) => format!(
				"{} drew {}",
				statics.player_name(*player_index),
				spec.label(),
			),
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
