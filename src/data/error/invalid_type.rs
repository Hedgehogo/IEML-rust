//! Type definition [`InvalidTypeError`]

use super::{super::node_type::NodeType, expected::Expected};
use std::fmt::{Debug, Display, Formatter};

/// Error type responsible for the discrepancy between the requested node type and the available one.
#[derive(PartialEq, Eq, Debug)]
pub struct InvalidTypeError {
    unexpected_type: NodeType,
    expected_types: Expected<NodeType>,
}

impl InvalidTypeError {
    pub fn new(unexpected_type: NodeType, expected_types: impl Into<Expected<NodeType>>) -> Self {
        Self {
            unexpected_type,
            expected_types: expected_types.into(),
        }
    }

    pub fn unexpected_type(&self) -> NodeType {
        self.unexpected_type
    }

    pub fn expected_type(&self) -> Expected<NodeType> {
        self.expected_types
    }
}

impl Display for InvalidTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: value of unexpected type {}", self.unexpected_type)?;
        if !self.expected_types.is_empty() {
            write!(f, ", one of these types was expected: {}", self.expected_types[0])?;
            for i in self.expected_types.into_iter().skip(1) {
                write!(f, ", {}", i)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for InvalidTypeError {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub type InvalidTypeError = MarkedError<super::InvalidTypeError>;
}
