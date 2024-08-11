//! Type definition [`UnknownTagError`]

use std::fmt;
use super::expected::Expected;

/// Error occurring when an extra key is detected in the map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownTagError {
    unexpected_tag: String,
    expected_tags: Expected<&'static str>,
}

impl UnknownTagError {
    pub fn new(unexpected_tag: String, expected_tags: impl Into<Expected<&'static str>>) -> Self {
        Self {
            unexpected_tag,
            expected_tags: expected_tags.into(),
        }
    }

    pub fn unexpected_tag(&self) -> &str {
        &self.unexpected_tag
    }

    pub fn expected_tags(&self) -> Expected<&'static str> {
        self.expected_tags
    }
}

impl fmt::Display for UnknownTagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: unexpected tag {:?}", self.unexpected_tag)?;
        match self.expected_tags {
            Expected::One(i) => write!(f, ", expected tag {:?}", i)?,
            Expected::Many(i) => {
                let mut iter = i.iter();
                if let Some(i) = iter.next() {
                    write!(f, ", one of these tags was expected: {:?}", i)?;
                    for i in iter {
                        write!(f, ", {:?}", i)?;
                    }
                }
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
