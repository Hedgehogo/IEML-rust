use std::collections::HashMap;
use crate::data::{
	node::Node,
	node_data::NodeData,
};

#[derive(Clone, Default, Eq)]
pub struct AnchorKeeper<'a> {
	pub anchors: HashMap<String, Node<'a>>,
	pub file_anchors: HashMap<String, Node<'a>>,
	parent: Option<&'a AnchorKeeper<'a>>,
}

impl PartialEq for AnchorKeeper<'_> {
	fn eq(&self, other: &Self) -> bool {
		return
			self.anchors == other.anchors &&
				self.file_anchors == other.anchors;
	}
}

impl<'a> AnchorKeeper<'a> {
	pub(crate) fn new() -> Self {
		return Default::default();
	}
	
	pub(crate) fn new_with_parent(parent: Option<&'a AnchorKeeper<'a>>) -> Self {
		return Self {
			anchors: Default::default(),
			file_anchors: Default::default(),
			parent,
		};
	}
	
	pub(crate) fn add(&mut self, key: String, node: Node<'a>) -> Option<Node<'a>> {
		self.anchors.insert(key, node)
	}
	
	pub(crate) fn add_to_file(&mut self, key: String, node: Node<'a>) -> Option<Node<'a>> {
		self.file_anchors.insert(key, node)
	}
	
	pub(crate) fn get(&'a self, key: &str) -> Option<&Node> {
		self.file_anchors.get(key).or_else(|| {
			self.anchors.get(key).or_else(|| {
				self.parent.and_then(|i| i.get(key))
			})
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn add_test() {
		let mut anchor_keeper = AnchorKeeper::new();
		assert!(anchor_keeper.add("key".to_string(), Node::new(NodeData::String("hello".to_string()), Default::default())).is_none());
		assert!(anchor_keeper.add("key".to_string(), Node::new(NodeData::String("hello".to_string()), Default::default())).is_some());
	}
	
	#[test]
	fn add_to_file_test() {
		let mut anchor_keeper = AnchorKeeper::new();
		assert!(anchor_keeper.add_to_file("key".to_string(), Node::new(NodeData::String("hello".to_string()), Default::default())).is_none());
		assert!(anchor_keeper.add_to_file("key".to_string(), Node::new(NodeData::String("hello".to_string()), Default::default())).is_some());
	}
	
	#[test]
	fn get_test() {
		let mut anchor_keeper = AnchorKeeper::new();
		todo!()
	}
}