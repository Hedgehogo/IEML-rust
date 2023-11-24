pub mod another_type;
pub mod failed_decode;
pub mod invalid_index;
pub mod invalid_key;
pub mod with_mark;

pub use another_type::AnotherTypeError;
pub use failed_decode::FailedDecodeError;
pub use invalid_index::InvalidIndexError;
pub use invalid_key::InvalidKeyError;

pub mod marked {
    use std::fmt::{Debug, Display, Formatter};

    pub use super::with_mark::WithMarkError;

    pub type AnotherTypeError = WithMarkError<super::AnotherTypeError>;
    pub type FailedDecodeError = WithMarkError<super::FailedDecodeError>;
    pub type InvalidIndexError = WithMarkError<super::InvalidIndexError>;
    pub type InvalidKeyError = WithMarkError<super::InvalidKeyError>;

    #[derive(Debug, PartialEq, Eq)]
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

    #[derive(Debug, PartialEq, Eq)]
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

    #[derive(Debug)]
    pub enum DecodeError {
        NodeAnotherType(AnotherTypeError),
        InvalidIndex(InvalidIndexError),
        InvalidKey(InvalidKeyError),
        FailedDecode(FailedDecodeError),
        Failed,
    }

    impl Display for DecodeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                DecodeError::NodeAnotherType(e) => write!(f, "{}", e),
                DecodeError::InvalidIndex(e) => write!(f, "{}", e),
                DecodeError::InvalidKey(e) => write!(f, "{}", e),
                DecodeError::FailedDecode(e) => write!(f, "{}", e),
                DecodeError::Failed => write!(f, ""),
            }
        }
    }

    impl From<AnotherTypeError> for DecodeError {
        fn from(value: AnotherTypeError) -> Self {
            DecodeError::NodeAnotherType(value)
        }
    }

    impl From<InvalidIndexError> for DecodeError {
        fn from(value: InvalidIndexError) -> Self {
            DecodeError::InvalidIndex(value)
        }
    }

    impl From<InvalidKeyError> for DecodeError {
        fn from(value: InvalidKeyError) -> Self {
            DecodeError::InvalidKey(value)
        }
    }

    impl From<FailedDecodeError> for DecodeError {
        fn from(value: FailedDecodeError) -> Self {
            DecodeError::FailedDecode(value)
        }
    }

    impl From<ListError> for DecodeError {
        fn from(value: ListError) -> Self {
            match value {
                ListError::NodeAnotherType(i) => DecodeError::NodeAnotherType(i),
                ListError::InvalidIndex(i) => DecodeError::InvalidIndex(i),
            }
        }
    }

    impl From<MapError> for DecodeError {
        fn from(value: MapError) -> Self {
            match value {
                MapError::NodeAnotherType(i) => DecodeError::NodeAnotherType(i),
                MapError::InvalidKey(i) => DecodeError::InvalidKey(i),
            }
        }
    }
}
