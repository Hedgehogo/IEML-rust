//! Type definition [`InvalidValueError`]

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

/// Deserialisation error type containing a child error - the reason.
#[derive(PartialEq, Eq, Debug)]
pub struct InvalidValueError<E> {
    expected: String,
    reason: Option<Box<marked::DeserializeError<E>>>,
}

impl<E> InvalidValueError<E> {
    pub fn new(expected: String, reason: Option<Box<marked::DeserializeError<E>>>) -> Self {
        Self { expected, reason }
    }

    pub fn expected(&self) -> &str {
        &self.expected
    }

    pub fn reason(&self) -> &Option<Box<marked::DeserializeError<E>>> {
        &self.reason
    }
}

impl<E: Display> Display for InvalidValueError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.reason {
            Some(i) => write!(f, "{}\nnote: when trying to receive {}", i.as_ref(), self.expected()),
            None => write!(f, "error: expected {}", self.expected()),
        }
    }
}

impl<E: Error> Error for InvalidValueError<E> {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub(super) use super::super::deserialize::marked::DeserializeError;

    pub type InvalidValueError<E> = MarkedError<super::InvalidValueError<E>>;
}
