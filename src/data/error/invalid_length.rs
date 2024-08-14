//! Type definition [`InvalidLengthError`]

use std::fmt::{Debug, Display, Formatter};

/// Error of index access to a list item.
#[derive(PartialEq, Eq, Debug)]
pub struct InvalidLengthError {
    length: usize,
}

impl InvalidLengthError {
    pub fn new(length: usize) -> Self {
        Self { length }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

impl Display for InvalidLengthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: list or map of unexpected length equal to {}", self.length)
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
        let error = InvalidLengthError::new(3);
        assert_eq!(
            error.to_string(),
            "error: list or map of unexpected length equal to 3"
        );
    }
}
