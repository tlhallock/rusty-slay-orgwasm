use log;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct SlayError {
	reason: String,
}

pub type SlayResult<T> = std::result::Result<T, SlayError>;

impl SlayError {
	pub fn n(reason: String) -> SlayError {
		log::info!("Reason: {}", reason);
		unreachable!();
		// SlayError {
		// 	reason: reason.to_string(),
		// }
	}
	pub fn new(reason: &'static str) -> SlayError {
		log::info!("Reason: {}", reason);
		unreachable!();
		// SlayError {
		// 	reason: reason.to_string(),
		// }
	}
}

impl fmt::Display for SlayError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Slay error: {}", self.reason)
	}
}
