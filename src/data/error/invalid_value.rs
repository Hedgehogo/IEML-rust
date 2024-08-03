//! Type definition [`InvalidValueError`]

use super::deserialize::DeserializeError;
use std::{
    any::type_name,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

/// Deserialisation error type containing a child error - the reason.
#[derive(PartialEq, Eq, Debug)]
pub struct InvalidValueError<E> {
    type_name: &'static str,
    reason: Box<marked::DeserializeError<E>>,
}

impl<E> InvalidValueError<E> {
    pub fn new<T>(reason: Box<marked::DeserializeError<E>>) -> Self {
        Self {
            type_name: type_name::<T>(),
            reason,
        }
    }

    pub fn type_name(&self) -> &'static str {
        self.type_name
    }

    pub fn reason(&self) -> &Box<marked::DeserializeError<E>> {
        &self.reason
    }
}

impl<E: Display> Display for InvalidValueError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.reason.data {
            DeserializeError::Failed => {}
            _ => write!(f, "{}\n", self.reason.data)?,
        }
        write!(f, "error: expected value of type '{}'", self.type_name())
    }
}

impl<E: Error> Error for InvalidValueError<E> {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub(super) use super::super::deserialize::marked::DeserializeError;

    pub type InvalidValueError<E> = MarkedError<super::InvalidValueError<E>>;
}
