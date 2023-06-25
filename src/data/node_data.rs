use std::{
	collections::HashMap,
	path::Path,
};
use crate::anchor_keeper::{AnchorKeeper};
use super::{
	node::Node,
	node_type::NodeType
};

pub type RawData = String;

pub type StringData = String;

pub type ListData<'a> = Vec<Node<'a>>;

pub type MapData<'a> = HashMap<String, Node<'a>>;

pub type Tag = String;

#[derive(Clone, PartialEq, Eq)]
pub struct TagData<'a> {
	pub data: Box<Node<'a>>,
	pub tag: Tag,
}

#[derive(Clone, Eq)]
pub struct FileData<'a> {
	pub data: Box<Node<'a>>,
	pub file_path: Box<Path>,
	pub anchor_keeper: AnchorKeeper<'a>,
}

impl PartialEq for FileData<'_> {
	fn eq(&self, other: &Self) -> bool {
		return
			self.data == other.data &&
			self.file_path == other.file_path &&
			self.anchor_keeper == other.anchor_keeper
	}
}

#[derive(Clone, Eq)]
pub struct AnchorData<'a> {
	pub name: String,
	pub keeper: &'a AnchorKeeper<'a>,
}

impl PartialEq for AnchorData<'_> {
	fn eq(&self, other: &Self) -> bool {
		return self.name == other.name;
	}
}

#[derive(Clone, Default, PartialEq, Eq)]
pub enum NodeData<'a> {
	#[default]
	Null,
	Raw(RawData),
	String(StringData),
	List(ListData<'a>),
	Map(MapData<'a>),
	Tag(TagData<'a>),
	File(FileData<'a>),
	TakeAnchor(AnchorData<'a>),
	GetAnchor(AnchorData<'a>),
}

impl NodeData<'_> {
	pub(crate) fn get_node_type(&self) -> NodeType {
		match self {
			NodeData::Null => NodeType::Null,
			NodeData::Raw(_) => NodeType::Raw,
			NodeData::String(_) => NodeType::String,
			NodeData::List(_) => NodeType::List,
			NodeData::Map(_) => NodeType::Map,
			NodeData::Tag(_) => NodeType::Tag,
			NodeData::File(_) => NodeType::File,
			NodeData::TakeAnchor(_) => NodeType::TakeAnchor,
			NodeData::GetAnchor(_) => NodeType::GetAnchor,
		}
	}
}