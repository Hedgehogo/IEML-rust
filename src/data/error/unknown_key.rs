//! Type definition [`UnknownKeyError`]

use super::expected::Expected;
use std::fmt;

/// Error occurring when an extra key is detected in the map.
#[derive(PartialEq, Eq, Debug)]
pub struct UnknownKeyError {
    unexpected_key: String,
    expected_keys: Expected<&'static str>,
}

impl UnknownKeyError {
    pub fn new(unexpected_key: String, expected_keys: impl Into<Expected<&'static str>>) -> Self {
        Self {
            unexpected_key,
            expected_keys: expected_keys.into(),
        }
    }

    pub fn unexpected_key(&self) -> &str {
        &self.unexpected_key
    }

    pub fn expected_keys(&self) -> Expected<&'static str> {
        self.expected_keys
    }
}

impl fmt::Display for UnknownKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error: map contains an extra key named {:?}",
            self.unexpected_key
        )?;
        self.expected_keys
            .display(f, "key", "keys", fmt::Debug::fmt)
    }
}

impl std::error::Error for UnknownKeyError {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub type UnknownKeyError = MarkedError<super::UnknownKeyError>;
}
