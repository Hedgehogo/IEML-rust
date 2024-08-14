//! Type definition [`InvalidTypeError`]

use super::{super::node_type::NodeType, expected::Expected};
use std::fmt;

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

    pub fn expected_types(&self) -> Expected<NodeType> {
        self.expected_types
    }
}

impl fmt::Display for InvalidTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error: value of unexpected type {}",
            self.unexpected_type
        )?;
        self.expected_types
            .display(f, "type", "types", fmt::Display::fmt)
    }
}

impl std::error::Error for InvalidTypeError {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub type InvalidTypeError = MarkedError<super::InvalidTypeError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let expected = &[NodeType::Tagged, NodeType::Anchor, NodeType::Document] as &[_];
        let error = InvalidTypeError::new(NodeType::Raw, expected);
        assert_eq!(
            error.to_string(),
            "error: value of unexpected type number, boolean, raw data, expected types: tagged, anchor, document"
        );
    }
}
