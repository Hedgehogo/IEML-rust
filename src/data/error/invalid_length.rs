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

    pub fn len(&self) -> usize {
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
    use super::super::with_mark::WithMarkError;

    pub type InvalidLengthError = WithMarkError<super::InvalidLengthError>;
}
