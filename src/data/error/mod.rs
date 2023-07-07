pub mod with_mark;
pub mod another_type;
pub mod failed_decode;
pub mod invalid_index;
pub mod invalid_key;

pub use another_type::AnotherTypeError;
pub use failed_decode::FailedDecodeError;
pub use invalid_index::InvalidIndexError;
pub use invalid_key::InvalidKeyError;

pub mod marked {
    use std::{
        fmt::{Debug, Display, Formatter},
        error::Error,
    };
    
    pub use super::with_mark::WithMarkError;
    
    pub type AnotherTypeError = WithMarkError<super::AnotherTypeError>;
    pub type FailedDecodeError<E> = WithMarkError<super::FailedDecodeError<E>>;
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
    pub enum DecodeError<E: Error + PartialEq + Eq> {
        NodeAnotherType(AnotherTypeError),
        InvalidIndex(InvalidIndexError),
        InvalidKey(InvalidKeyError),
        FailedDecode(FailedDecodeError<E>),
        Other(E),
        Failed,
    }
    
    impl<E: Error + PartialEq + Eq> Display for DecodeError<E> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                DecodeError::NodeAnotherType(e) => write!(f, "{}", e),
                DecodeError::InvalidIndex(e) => write!(f, "{}", e),
                DecodeError::InvalidKey(e) => write!(f, "{}", e),
                DecodeError::FailedDecode(e) => write!(f, "{}", e),
                DecodeError::Other(e) => write!(f, "{}", e),
                DecodeError::Failed => write!(f, "")
            }
        }
    }
    
    impl<E: Error + PartialEq + Eq> From<AnotherTypeError> for DecodeError<E> {
        fn from(value: AnotherTypeError) -> Self {
            DecodeError::NodeAnotherType(value)
        }
    }
    
    impl<E: Error + PartialEq + Eq> From<InvalidIndexError> for DecodeError<E> {
        fn from(value: InvalidIndexError) -> Self {
            DecodeError::InvalidIndex(value)
        }
    }
    
    impl<E: Error + PartialEq + Eq> From<InvalidKeyError> for DecodeError<E> {
        fn from(value: InvalidKeyError) -> Self {
            DecodeError::InvalidKey(value)
        }
    }
    
    impl<E: Error + PartialEq + Eq> From<FailedDecodeError<E>> for DecodeError<E> {
        fn from(value: FailedDecodeError<E>) -> Self {
            DecodeError::FailedDecode(value)
        }
    }
    
    impl<E: Error + PartialEq + Eq> From<ListError> for DecodeError<E> {
        fn from(value: ListError) -> Self {
            match value {
                ListError::NodeAnotherType(i) => DecodeError::NodeAnotherType(i),
                ListError::InvalidIndex(i) => DecodeError::InvalidIndex(i),
            }
        }
    }
    
    impl<E: Error + PartialEq + Eq> From<MapError> for DecodeError<E> {
        fn from(value: MapError) -> Self {
            match value {
                MapError::NodeAnotherType(i) => DecodeError::NodeAnotherType(i),
                MapError::InvalidKey(i) => DecodeError::InvalidKey(i),
            }
        }
    }
}


