use std::fmt::{Display, Formatter};

/// Error of key access to a map item.
#[derive(PartialEq, Eq, Debug)]
pub struct MissingKeyError {
    expected_key: String,
}

impl MissingKeyError {
    pub fn new(expected_key: String) -> Self {
        Self { expected_key }
    }

    pub fn expected_key(&self) -> &str {
        &self.expected_key
    }
}

impl Display for MissingKeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: map that does not contain a key named {:?}", self.expected_key)
    }
}

impl std::error::Error for MissingKeyError {}

pub mod marked {
    use super::super::with_mark::WithMarkError;

    pub type MissingKeyError = WithMarkError<super::MissingKeyError>;
}
