//! Type definition [`UnknownRawError`]

use super::{expected::Expected, UnknownTagError};
use std::fmt;

/// Error occurring when a raw data content does not match any of the expected ones.
#[derive(PartialEq, Eq, Debug)]
pub struct UnknownRawError {
    unexpected_raw: String,
    expected_raw: Expected<&'static str>,
}

impl UnknownRawError {
    pub fn new(unexpected_raw: String, expected_raw: impl Into<Expected<&'static str>>) -> Self {
        Self {
            unexpected_raw,
            expected_raw: expected_raw.into(),
        }
    }

    pub fn unexpected_raw(&self) -> &str {
        &self.unexpected_raw
    }

    pub fn expected_raw(&self) -> Expected<&'static str> {
        self.expected_raw
    }

    pub fn split(self) -> (String, Expected<&'static str>) {
        (self.unexpected_raw, self.expected_raw)
    }
}

impl fmt::Display for UnknownRawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error: unexpected raw data content {:?}",
            self.unexpected_raw
        )?;
        self.expected_raw
            .display(f, "raw data content", "raw data content", fmt::Debug::fmt)
    }
}

impl std::error::Error for UnknownRawError {}

impl From<UnknownTagError> for UnknownRawError {
    fn from(value: UnknownTagError) -> Self {
        let (unexpected_raw, expected_raw) = value.split();
        Self::new(unexpected_raw, expected_raw)
    }
}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub type UnknownRawError = MarkedError<super::UnknownRawError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let error = UnknownRawError::new("Ok".into(), &["Some", "None"] as &[_]);
        assert_eq!(
            error.to_string(),
            r#"error: unexpected raw data content "Ok", expected raw data content: "Some", "None""#
        );
    }
}
