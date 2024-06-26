use crate::data::error::marked;
use std::{
    any::type_name,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

#[derive(PartialEq, Eq, Debug)]
pub struct FailedDeserializeError<E: Error + PartialEq + Eq> {
    type_name: &'static str,
    reason: Box<marked::DeserializeError<E>>,
}

impl<E: Error + PartialEq + Eq> FailedDeserializeError<E> {
    pub fn new<T>(reason: Box<marked::DeserializeError<E>>) -> Self {
        Self {
            type_name: type_name::<T>(),
            reason,
        }
    }

    pub fn get_type_name(&self) -> &'static str {
        self.type_name
    }

    pub fn get_reason(&self) -> &Box<marked::DeserializeError<E>> {
        &self.reason
    }
}

impl<E: Error + PartialEq + Eq> Display for FailedDeserializeError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self.reason {
            marked::DeserializeError::Failed => {
                write!(f, "Failed to convert view to '{}'.", self.get_type_name())
            }
            _ => write!(
                f,
                "Failed to convert view to '{}', because:\n{}",
                self.get_type_name(),
                *self.reason
            ),
        }
    }
}

impl<E: Error + PartialEq + Eq> Error for FailedDeserializeError<E> {}
