pub mod another_type;
pub mod failed_deserialize;
pub mod invalid_index;
pub mod invalid_key;
pub mod with_mark;

pub use another_type::AnotherTypeError;
pub use failed_deserialize::FailedDeserializeError;
pub use invalid_index::InvalidIndexError;
pub use invalid_key::InvalidKeyError;

pub mod marked {
    use std::{
        error::Error,
        fmt::{Debug, Display, Formatter},
    };

    pub use super::with_mark::WithMarkError;

    pub type AnotherTypeError = WithMarkError<super::AnotherTypeError>;
    pub type FailedDeserializeError<E> = WithMarkError<super::FailedDeserializeError<E>>;
    pub type InvalidIndexError = WithMarkError<super::InvalidIndexError>;
    pub type InvalidKeyError = WithMarkError<super::InvalidKeyError>;

    #[derive(PartialEq, Eq, Debug)]
    pub enum ListError {
        NodeAnotherType(AnotherTypeError),
        InvalidIndex(InvalidIndexError),
    }

    impl From<AnotherTypeError> for ListError {
        fn from(value: AnotherTypeError) -> Self {
            ListError::NodeAnotherType(value)
        }
    }

    impl From<InvalidIndexError> for ListError {
        fn from(value: InvalidIndexError) -> Self {
            ListError::InvalidIndex(value)
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum MapError {
        NodeAnotherType(AnotherTypeError),
        InvalidKey(InvalidKeyError),
    }

    impl From<AnotherTypeError> for MapError {
        fn from(value: AnotherTypeError) -> Self {
            MapError::NodeAnotherType(value)
        }
    }

    impl From<InvalidKeyError> for MapError {
        fn from(value: InvalidKeyError) -> Self {
            MapError::InvalidKey(value)
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum DeserializeError<E: Error + PartialEq + Eq> {
        NodeAnotherType(AnotherTypeError),
        InvalidIndex(InvalidIndexError),
        InvalidKey(InvalidKeyError),
        FailedDecode(FailedDeserializeError<E>),
        Other(E),
        Failed,
    }

    impl<E: Error + PartialEq + Eq> Display for DeserializeError<E> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                DeserializeError::NodeAnotherType(e) => write!(f, "{}", e),
                DeserializeError::InvalidIndex(e) => write!(f, "{}", e),
                DeserializeError::InvalidKey(e) => write!(f, "{}", e),
                DeserializeError::FailedDecode(e) => write!(f, "{}", e),
                DeserializeError::Other(e) => write!(f, "{}", e),
                DeserializeError::Failed => write!(f, ""),
            }
        }
    }

    impl<E: Error + PartialEq + Eq> From<AnotherTypeError> for DeserializeError<E> {
        fn from(value: AnotherTypeError) -> Self {
            DeserializeError::NodeAnotherType(value)
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
            DeserializeError::FailedDecode(value)
        }
    }

    impl<E: Error + PartialEq + Eq> From<ListError> for DeserializeError<E> {
        fn from(value: ListError) -> Self {
            match value {
                ListError::NodeAnotherType(i) => DeserializeError::NodeAnotherType(i),
                ListError::InvalidIndex(i) => DeserializeError::InvalidIndex(i),
            }
        }
    }

    impl<E: Error + PartialEq + Eq> From<MapError> for DeserializeError<E> {
        fn from(value: MapError) -> Self {
            match value {
                MapError::NodeAnotherType(i) => DeserializeError::NodeAnotherType(i),
                MapError::InvalidKey(i) => DeserializeError::InvalidKey(i),
            }
        }
    }
}
