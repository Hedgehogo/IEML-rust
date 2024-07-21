use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub use super::with_mark::WithMarkError;

pub type AnotherTypeError = WithMarkError<super::AnotherTypeError>;
pub type FailedDeserializeError<E> = WithMarkError<super::FailedDeserializeError<E>>;
pub type InvalidIndexError = WithMarkError<super::InvalidIndexError>;
pub type InvalidKeyError = WithMarkError<super::InvalidKeyError>;

/// General type of deserialisation error
#[derive(PartialEq, Eq, Debug)]
pub enum DeserializeError<E: Error + PartialEq + Eq> {
    /// Node access error due to mismatch between requested type and available type
    ViewAnotherType(AnotherTypeError),
    /// Node access error due to insufficient number of items in the list
    InvalidIndex(InvalidIndexError),
    /// Node access error due to missing key in the map
    InvalidKey(InvalidKeyError),
    /// Deserialisation error resulting from another error
    FailedDeserialize(FailedDeserializeError<E>),
    /// Additional error type for the possibility of extending this type
    Other(E),
    /// Error without specifying any reason
    Failed,
}

impl<E: Error + PartialEq + Eq> Display for DeserializeError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeError::ViewAnotherType(e) => write!(f, "{}", e),
            DeserializeError::InvalidIndex(e) => write!(f, "{}", e),
            DeserializeError::InvalidKey(e) => write!(f, "{}", e),
            DeserializeError::FailedDeserialize(e) => write!(f, "{}", e),
            DeserializeError::Other(e) => write!(f, "{}", e),
            DeserializeError::Failed => write!(f, ""),
        }
    }
}

impl<E: Error + PartialEq + Eq> From<AnotherTypeError> for DeserializeError<E> {
    fn from(value: AnotherTypeError) -> Self {
        DeserializeError::ViewAnotherType(value)
    }
}

impl<E: Error + PartialEq + Eq> From<InvalidIndexError> for DeserializeError<E> {
    fn from(value: InvalidIndexError) -> Self {
        DeserializeError::InvalidIndex(value)
    }
}

impl<E: Error + PartialEq + Eq> From<InvalidKeyError> for DeserializeError<E> {
    fn from(value: InvalidKeyError) -> Self {
        DeserializeError::InvalidKey(value)
    }
}

impl<E: Error + PartialEq + Eq> From<FailedDeserializeError<E>> for DeserializeError<E> {
    fn from(value: FailedDeserializeError<E>) -> Self {
        DeserializeError::FailedDeserialize(value)
    }
}
