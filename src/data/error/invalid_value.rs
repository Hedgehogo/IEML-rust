//! Type definition [`InvalidValueError`]

use super::deserialize::DeserializeError;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

/// Deserialisation error type containing a child error - the reason.
#[derive(PartialEq, Eq, Debug)]
pub struct InvalidValueError<E> {
    expected: String,
    reason: Box<marked::DeserializeError<E>>,
}

impl<E> InvalidValueError<E> {
    pub fn new(expected: String, reason: Box<marked::DeserializeError<E>>) -> Self {
        Self { expected, reason }
    }

    pub fn new_expected(expected: String) -> Self {
        let deserialize = marked::DeserializeError::failed(Default::default());
        Self::new(expected, Box::new(deserialize))
    }

    pub fn expected(&self) -> &str {
        &self.expected
    }

    pub fn reason(&self) -> &marked::DeserializeError<E> {
        &self.reason
    }
}

impl<E: Display> Display for InvalidValueError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.reason.data {
            DeserializeError::Failed => {}
            _ => writeln!(f, "{}", self.reason.data)?,
        }
        write!(f, "error: expected {}", self.expected())
    }
}

impl<E: Error> Error for InvalidValueError<E> {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub(super) use super::super::deserialize::marked::DeserializeError;

    pub type InvalidValueError<E> = MarkedError<super::InvalidValueError<E>>;
}
