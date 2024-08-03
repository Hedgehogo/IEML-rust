//! Type definition [`UnknownTagError`]

use std::fmt::{Display, Formatter};

/// Error occurring when an extra key is detected in the map.
#[derive(PartialEq, Eq, Debug)]
pub struct UnknownTagError {
    unexpected_tag: String,
    expected_tags: &'static [&'static str],
}

impl UnknownTagError {
    pub fn new(unexpected_tag: String, expected_tags: &'static [&'static str]) -> Self {
        Self { unexpected_tag, expected_tags }
    }

    pub fn unexpected_tag(&self) -> &str {
        &self.unexpected_tag
    }

    pub fn expected_tags(&self) -> &'static [&'static str] {
        self.expected_tags
    }
}

impl Display for UnknownTagError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: unexpected tag {}", self.unexpected_tag)?;
        if !self.expected_tags.is_empty() {
            write!(f, ", one of these tags was expected: {}", self.expected_tags[0])?;
            for i in self.expected_tags.iter().skip(1) {
                write!(f, ", {}", i)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for UnknownTagError {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub type UnknownTagError = MarkedError<super::UnknownTagError>;
}
