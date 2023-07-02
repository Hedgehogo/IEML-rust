pub mod with_mark;
pub mod node;
pub mod node_another_type;
pub mod failed_decode;
pub mod invalid_index;
pub mod invalid_key;

pub type NodeAnotherTypeError = node_another_type::Error;
pub type FailedDecodeError = failed_decode::Error;
pub type InvalidIndexError = invalid_index::Error;
pub type InvalidKeyError = invalid_key::Error;

pub mod marked {
    pub type Error<T> = super::with_mark::Error<T>;
    
    pub type NodeAnotherTypeError = Error<super::NodeAnotherTypeError>;
    pub type FailedDecodeError = Error<super::FailedDecodeError>;
    pub type InvalidIndexError = Error<super::InvalidIndexError>;
    pub type InvalidKeyError = Error<super::InvalidKeyError>;
    
    pub enum ListError {
        NodeAnotherType(NodeAnotherTypeError),
        InvalidIndex(InvalidIndexError),
    }
    
    impl From<NodeAnotherTypeError> for ListError {
        fn from(value: NodeAnotherTypeError) -> Self {
            ListError::NodeAnotherType(value)
        }
    }
    
    impl From<InvalidIndexError> for ListError {
        fn from(value: InvalidIndexError) -> Self {
            ListError::InvalidIndex(value)
        }
    }
    
    pub enum MapError {
        NodeAnotherType(NodeAnotherTypeError),
        InvalidKey(InvalidKeyError),
    }
    
    impl From<NodeAnotherTypeError> for MapError {
        fn from(value: NodeAnotherTypeError) -> Self {
            MapError::NodeAnotherType(value)
        }
    }
    
    impl From<InvalidKeyError> for MapError {
        fn from(value: InvalidKeyError) -> Self {
            MapError::InvalidKey(value)
        }
    }
    
    pub enum DecodeError {
        NodeAnotherType(NodeAnotherTypeError),
        InvalidIndex(InvalidIndexError),
        InvalidKey(InvalidKeyError),
        FailedDecode(FailedDecodeError),
        Other(Box<dyn std::error::Error>),
        Failed,
    }
    
    impl From<NodeAnotherTypeError> for DecodeError {
        fn from(value: NodeAnotherTypeError) -> Self {
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
    
    impl From<Box<dyn std::error::Error>> for DecodeError {
        fn from(value: Box<dyn std::error::Error>) -> Self {
            DecodeError::Other(value)
        }
    }
}


