//! Type definition [`UnknownKeyError`]

use std::fmt::{Display, Formatter};

/// Error occurring when an extra key is detected in the map.
#[derive(PartialEq, Eq, Debug)]
pub struct UnknownKeyError {
    unexpected_key: String,
    expected_keys: &'static [&'static str],
}

impl UnknownKeyError {
    pub fn new(unexpected_key: String, expected_keys: &'static [&'static str]) -> Self {
        Self { unexpected_key, expected_keys }
    }

    pub fn unexpected_key(&self) -> &str {
        &self.unexpected_key
    }

    pub fn expected_keys(&self) -> &'static [&'static str] {
        self.expected_keys
    }
}

impl Display for UnknownKeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: map contains an extra key named {:?}", self.unexpected_key)?;
        if !self.expected_keys.is_empty() {
            write!(f, ", one of these keys was expected: {:?}", self.expected_keys[0])?;
            for i in self.expected_keys.iter().skip(1) {
                write!(f, ", {:?}", i)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for UnknownKeyError {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub type UnknownKeyError = MarkedError<super::UnknownKeyError>;
}
