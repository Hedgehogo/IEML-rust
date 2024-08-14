//! Type definition [`MissingKeyError`]

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
        write!(
            f,
            "error: map that does not contain a key named {:?}",
            self.expected_key
        )
    }
}

impl std::error::Error for MissingKeyError {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub type MissingKeyError = MarkedError<super::MissingKeyError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let error = MissingKeyError::new("field".into());
        assert_eq!(
            error.to_string(),
            r#"error: map that does not contain a key named "field""#
        );
    }
}
