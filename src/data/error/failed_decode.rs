use std::{
    any::type_name,
    fmt::{Debug, Display, Formatter},
    error::Error,
};
use crate::data::error::marked;

#[derive(PartialEq, Eq, Debug)]
pub struct FailedDecodeError<E: Error + PartialEq + Eq> {
    type_name: &'static str,
    reason: Box<marked::DecodeError<E>>,
}

impl<E: Error + PartialEq + Eq> FailedDecodeError<E> {
    pub fn new<T>(reason: Box<marked::DecodeError<E>>) -> Self {
        Self { type_name: type_name::<T>(), reason }
    }
    
    pub fn get_type_name(&self) -> &'static str {
        self.type_name
    }
    
    pub fn get_reason(&self) -> &Box<marked::DecodeError<E>> {
        &self.reason
    }
}

impl<E: Error + PartialEq + Eq> Display for FailedDecodeError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self.reason {
            marked::DecodeError::Failed => write!(f, "Failed to convert node to '{}'.", self.get_type_name()),
            _ => write!(f, "Failed to convert node to '{}', because:\n{}", self.get_type_name(), *self.reason),
        }
    }
}

impl<E: Error + PartialEq + Eq> Error for FailedDecodeError<E> {}