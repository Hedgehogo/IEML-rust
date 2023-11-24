use crate::data::error::marked;
use std::{
    any::type_name,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug)]
pub struct FailedDecodeError {
    pub type_name: &'static str,
    pub reason: Box<marked::DecodeError>,
}

impl FailedDecodeError {
    pub fn new<T>(reason: Box<marked::DecodeError>) -> Self {
        Self {
            type_name: type_name::<T>(),
            reason,
        }
    }
}

impl Display for FailedDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self.reason {
            marked::DecodeError::Failed => {
                write!(f, "Failed to convert node to '{}'.", self.type_name)
            }
            _ => write!(
                f,
                "Failed to convert node to '{}', because:\n{}",
                self.type_name, *self.reason
            ),
        }
    }
}

impl Error for FailedDecodeError {}
