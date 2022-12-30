use chrono::DateTime;
use chrono::Utc;

use chrono::Duration;

pub fn get_action_point_choice_deadline() -> Option<DateTime<Utc>> {
	Some(chrono::offset::Utc::now() + Duration::seconds(30))
}

pub fn get_discard_deadline() -> Option<DateTime<Utc>> {
	Some(chrono::offset::Utc::now() + Duration::seconds(30))
}

pub fn get_sacrifice_deadline() -> Option<DateTime<Utc>> {
	Some(chrono::offset::Utc::now() + Duration::seconds(30))
}

pub fn get_roll_deadline() -> Option<DateTime<Utc>> {
	Some(chrono::offset::Utc::now() + Duration::seconds(30))
}

pub fn get_challenge_deadline() -> Option<DateTime<Utc>> {
	Some(chrono::offset::Utc::now() + Duration::seconds(30))
}
pub fn get_offer_challenges_deadline() -> Option<DateTime<Utc>> {
	Some(chrono::offset::Utc::now() + Duration::seconds(30))
}
