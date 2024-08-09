//! Type definition [`DeserializeError`]

use super::*;
use crate::error::custom::CustomError;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

/// General type of deserialisation error
#[derive(PartialEq, Eq, Debug)]
pub enum DeserializeError<E> {
    /// Occurs when the reader receives a different type than expected.
    InvalidType(InvalidTypeError),
    /// Occurs when the reader receives a value of the right type but wrong for another reason.
    InvalidValue(InvalidValueError<E>),
    /// Occurs when reading a list or map, but the input data contains too many or too few elements..
    InvalidLength(InvalidLengthError),
    /// Occurs when reading a tagged node, but the tag does not match any of the expected ones.
    UnknownTag(UnknownTagError),
    /// Occurs when reading a map, but the input data contains the extra key.
    UnknownKey(UnknownKeyError),
    /// Occurs when reading a map, but the input data does not contain the requested key.
    MissingKey(MissingKeyError),
    /// Occurs when the cause of an error cannot be expressed by this type.
    Failed,
    /// Additional error type for the possibility of extending this type
    Custom(E),
}

impl<E: Display> Display for DeserializeError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeError::InvalidType(i) => write!(f, "{}", i),
            DeserializeError::InvalidValue(i) => write!(f, "{}", i),
            DeserializeError::InvalidLength(i) => write!(f, "{}", i),
            DeserializeError::UnknownTag(i) => write!(f, "{}", i),
            DeserializeError::UnknownKey(i) => write!(f, "{}", i),
            DeserializeError::MissingKey(i) => write!(f, "{}", i),
            DeserializeError::Failed => write!(f, "failed"),
            DeserializeError::Custom(i) => write!(f, "{}", i),
        }
    }
}

impl<E: Debug + Display> Error for DeserializeError<E> {}

impl<E> From<InvalidTypeError> for DeserializeError<E> {
    fn from(value: InvalidTypeError) -> Self {
        DeserializeError::InvalidType(value)
    }
}

impl<E> From<InvalidValueError<E>> for DeserializeError<E> {
    fn from(value: InvalidValueError<E>) -> Self {
        DeserializeError::InvalidValue(value)
    }
}

impl<E> From<InvalidLengthError> for DeserializeError<E> {
    fn from(value: InvalidLengthError) -> Self {
        DeserializeError::InvalidLength(value)
    }
}

impl<E> From<UnknownTagError> for DeserializeError<E> {
    fn from(value: UnknownTagError) -> Self {
        DeserializeError::UnknownTag(value)
    }
}

impl<E> From<UnknownKeyError> for DeserializeError<E> {
    fn from(value: UnknownKeyError) -> Self {
        DeserializeError::UnknownKey(value)
    }
}

impl<E> From<MissingKeyError> for DeserializeError<E> {
    fn from(value: MissingKeyError) -> Self {
        DeserializeError::MissingKey(value)
    }
}

impl From<CustomError> for DeserializeError<CustomError> {
    fn from(value: CustomError) -> Self {
        DeserializeError::Custom(value)
    }
}

pub mod marked {
    use super::super::super::mark::Mark;
    use super::super::marked::*;
    use crate::error::{custom::marked::CustomError, marked::MarkedError};

    pub type DeserializeError<E> = MarkedError<super::DeserializeError<E>>;

    impl<E> DeserializeError<E> {
        pub fn failed(mark: Mark) -> Self {
            MarkedError::new(mark, super::DeserializeError::Failed)
        }
    }

    impl<E> From<InvalidTypeError> for DeserializeError<E> {
        fn from(value: InvalidTypeError) -> Self {
            MarkedError::new(value.mark, super::DeserializeError::InvalidType(value.data))
        }
    }

    impl<E> From<InvalidValueError<E>> for DeserializeError<E> {
        fn from(value: InvalidValueError<E>) -> Self {
            MarkedError::new(
                value.mark,
                super::DeserializeError::InvalidValue(value.data),
            )
        }
    }

    impl<E> From<InvalidLengthError> for DeserializeError<E> {
        fn from(value: InvalidLengthError) -> Self {
            MarkedError::new(
                value.mark,
                super::DeserializeError::InvalidLength(value.data),
            )
        }
    }

    impl<E> From<UnknownTagError> for DeserializeError<E> {
        fn from(value: UnknownTagError) -> Self {
            MarkedError::new(value.mark, super::DeserializeError::UnknownTag(value.data))
        }
    }

    impl<E> From<UnknownKeyError> for DeserializeError<E> {
        fn from(value: UnknownKeyError) -> Self {
            MarkedError::new(value.mark, super::DeserializeError::UnknownKey(value.data))
        }
    }

    impl<E> From<MissingKeyError> for DeserializeError<E> {
        fn from(value: MissingKeyError) -> Self {
            MarkedError::new(value.mark, super::DeserializeError::MissingKey(value.data))
        }
    }

    impl From<CustomError> for DeserializeError<super::CustomError> {
        fn from(value: CustomError) -> Self {
            MarkedError::new(value.mark, super::DeserializeError::Custom(value.data))
        }
    }
}
