use std::fmt::{Debug, Display, Formatter};
use super::super::node_type::NodeType;

#[derive(Debug)]
pub struct Error {
	requested_type: NodeType,
	node_type: NodeType,
}

impl Error {
	pub fn new(requested_type: NodeType, node_type: NodeType) -> Self {
		Self{requested_type, node_type}
	}
	
	pub fn get_requested_type(&self) -> NodeType {
		self.requested_type
	}
	
	pub fn get_node_type(&self) -> NodeType {
		self.node_type
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Node of the '{:?}' type cannot be converted to a value of the '{:?}' type.", self.node_type, self.requested_type)
	}
}

impl std::error::Error for Error {}