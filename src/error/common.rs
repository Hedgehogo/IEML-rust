//! Type definition [`CommonError`]

use crate::data::error::DeserializeError;
use crate::de::parse::error::Error as ParseError;
use std::fmt;

/// An error type that combines deserialisation errors from IEML input to IEML data structure and from IEML data structure to Rust data structure.
#[derive(PartialEq, Eq, Debug)]
pub enum CommonError<E> {
    /// Deserialisation error from IEML input to IEML data structure.
    Parse(ParseError),
    /// Deserialisation error from IEML data structure to Rust data structure.
    Deserialize(DeserializeError<E>),
}

impl<E> From<DeserializeError<E>> for CommonError<E> {
    fn from(value: DeserializeError<E>) -> Self {
        CommonError::Deserialize(value)
    }
}

impl<E> From<ParseError> for CommonError<E> {
    fn from(value: ParseError) -> Self {
        CommonError::Parse(value)
    }
}

impl<E: fmt::Display> fmt::Display for CommonError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommonError::Parse(i) => write!(f, "{i}"),
            CommonError::Deserialize(i) => write!(f, "{i}"),
        }
    }
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for CommonError<E> {}

pub mod marked {
    use super::super::marked::MarkedError;
    use crate::data::error::marked::DeserializeError;
    use crate::de::parse::error::marked::Error as ParseError;

    pub type CommonError<E> = MarkedError<super::CommonError<E>>;

    impl<E> From<DeserializeError<E>> for CommonError<E> {
        fn from(value: DeserializeError<E>) -> Self {
            MarkedError::new(value.mark, super::CommonError::Deserialize(value.data))
        }
    }

    impl<E> From<ParseError> for CommonError<E> {
        fn from(value: ParseError) -> Self {
            MarkedError::new(value.mark, super::CommonError::Parse(value.data))
        }
    }
}
