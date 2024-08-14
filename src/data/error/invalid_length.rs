//! Type definition [`InvalidLengthError`]

use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum Origin {
    List,
    Map,
}

/// Error of index access to a list item.
#[derive(Debug, PartialEq, Eq)]
pub struct InvalidLengthError {
    length: usize,
    origin: Option<Origin>,
    expected: Option<usize>,
}

impl InvalidLengthError {
    pub fn new(length: usize, origin: Option<Origin>, expected: Option<usize>) -> Self {
        Self {
            length,
            origin,
            expected,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

impl Display for InvalidLengthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let origin = match self.origin {
            Some(Origin::List) => "list",
            Some(Origin::Map) => "map",
            None => "list or map",
        };

        write!(
            f,
            "error: {} of unexpected length equal to {}",
            origin, self.length
        )?;
        if let Some(i) = self.expected {
            write!(f, ", expected length equal to {}", i)?;
        }

        Ok(())
    }
}

impl std::error::Error for InvalidLengthError {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub type InvalidLengthError = MarkedError<super::InvalidLengthError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let error = InvalidLengthError::new(3, None, None);
        assert_eq!(
            error.to_string(),
            "error: list or map of unexpected length equal to 3"
        );

        let error = InvalidLengthError::new(3, Some(Origin::List), None);
        assert_eq!(
            error.to_string(),
            "error: list of unexpected length equal to 3"
        );

        let error = InvalidLengthError::new(3, Some(Origin::Map), None);
        assert_eq!(
            error.to_string(),
            "error: map of unexpected length equal to 3"
        );

        let error = InvalidLengthError::new(3, Some(Origin::Map), Some(2));
        assert_eq!(
            error.to_string(),
            "error: map of unexpected length equal to 3, expected length equal to 2"
        );
    }
}
