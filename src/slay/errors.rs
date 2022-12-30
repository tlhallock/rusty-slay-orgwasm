use std::fmt;

#[derive(Debug)]
pub struct SlayError {
    reason: String,
}

pub type SlayResult<T> = std::result::Result<T, SlayError>;

impl SlayError {
    pub fn new(reason: &'static str) -> SlayError {
        SlayError {
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for SlayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Slay error: {}", self.reason)
    }
}
