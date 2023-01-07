use chrono::DateTime;
use chrono::Utc;

use chrono::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineCompletion {
	pub percent_complete: f64,
	pub seconds_remaining: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Timeline {
	// #[serde(with = "ts_milliseconds_option")]
	begin_time: DateTime<Utc>,
	duration_seconds: Option<i64>,
	// pub deadline: Option<DateTime<Utc>>,
}

impl Timeline {
	pub fn new(duration_seconds: Option<i64>) -> Self {
		let now = chrono::offset::Utc::now();
		Self {
			begin_time: now,
			duration_seconds,
			// deadline: Some(now + Duration::seconds(duration_seconds))
		}
	}

	fn compute_deadline(&self) -> Option<DateTime<Utc>> {
		self
			.duration_seconds
			.map(|duration_seconds| self.begin_time + Duration::seconds(duration_seconds))
	}

	pub fn completion(&self) -> Option<TimelineCompletion> {
		let now = chrono::offset::Utc::now();
		self.compute_deadline().map(|deadline| TimelineCompletion {
			percent_complete: (now - self.begin_time).num_milliseconds() as f64
				/ (deadline - self.begin_time).num_milliseconds() as f64,
			seconds_remaining: (deadline - now).num_milliseconds() as f64 / 1000f64,
		})
	}

	pub(crate) fn reset(&mut self) {
		self.begin_time = chrono::offset::Utc::now();
	}
}

pub fn current_time() -> DateTime<Utc> {
	chrono::offset::Utc::now()
}

pub fn get_action_point_choice_deadline() -> Timeline {
	Timeline::new(Some(30))
}

pub fn get_refactor_me_deadline() -> Timeline {
	Timeline::new(Some(30))
}
pub fn get_discard_deadline() -> Timeline {
	Timeline::new(Some(30))
}

pub fn get_sacrifice_deadline() -> Timeline {
	Timeline::new(Some(30))
}

pub fn get_roll_deadline() -> Timeline {
	Timeline::new(Some(30))
}

pub fn get_challenge_deadline() -> Timeline {
	Timeline::new(Some(30))
}
pub fn get_offer_challenges_deadline() -> Timeline {
	Timeline::new(Some(30))
}
