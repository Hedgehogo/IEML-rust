use super::super::node_type::NodeType;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct AnotherTypeError {
    pub requested_type: NodeType,
    pub node_type: NodeType,
}

impl AnotherTypeError {
    pub fn new(requested_type: NodeType, node_type: NodeType) -> Self {
        Self {
            requested_type,
            node_type,
        }
    }
}

impl Display for AnotherTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node of the '{:?}' type cannot be converted to a value of the '{:?}' type.",
            self.node_type, self.requested_type
        )
    }
}

impl std::error::Error for AnotherTypeError {}
