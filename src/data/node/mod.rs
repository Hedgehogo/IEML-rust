mod get_from;
mod decode;

use std::{
	iter::Map,
	path::Path,
};
use crate::anchor_keeper::AnchorKeeper;
use super::{
	mark::Mark,
	node_type::NodeType,
	node_data::{
		NodeData,
		RawData,
		StringData,
		ListData,
		MapData,
		Tag,
	},
	error::{
		NodeAnotherTypeError,
		FailedDecodeDataError,
		InvalidIndexError,
		InvalidKeyError,
		marked,
	},
};
use decode::Decode;

pub trait NodeIndex {
	type Error;
	
	fn at<'a>(node: &'a Node<'a>, index: Self) -> Result<&'a Node<'a>, Self::Error>;
}

impl NodeIndex for usize {
	type Error = marked::ListError;
	
	fn at<'a>(node: &'a Node<'a>, index: Self) -> Result<&'a Node<'a>, Self::Error> {
		node.at_index(index)
	}
}

impl NodeIndex for &str {
	type Error = marked::MapError;
	
	fn at<'a>(node: &'a Node<'a>, index: Self) -> Result<&'a Node<'a>, Self::Error> {
		node.at_key(index)
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct Node<'a> {
	data: NodeData<'a>,
	mark: Mark,
}

macro_rules! get_typed_data_or_error {
	($expr:expr, $type:ident) => {
		match &$expr.data {
			NodeData::$type(i) => Ok(i),
			_ => Err($expr.make_error(NodeAnotherTypeError::new(NodeType::$type, $expr.get_type()))),
		}
	};
}

impl<'a> Node<'a> {
	pub fn new(data: NodeData<'a>, mark: Mark) -> Node<'a> {
		return Self{data, mark};
	}
	
	/// Gets the node mark.
	pub fn get_mark(&'a self) -> Mark {
		self.mark
	}
	
	/// Gets the node type.
	pub fn get_type(&'a self) -> NodeType {
		self.data.get_node_type()
	}
	
	/// Asks if the node is Null.
	pub fn is_null(&'a self) -> bool {
		matches!(self.get_clear().data, NodeData::Null)
	}
	
	/// Asks if the node is Raw.
	pub fn is_raw(&'a self) -> bool {
		matches!(self.get_clear().data, NodeData::Raw(_))
	}
	
	/// Asks if the node is String.
	pub fn is_string(&'a self) -> bool {
		matches!(self.get_clear().data, NodeData::String(_))
	}
	
	/// Asks if the node is List.
	pub fn is_list(&'a self) -> bool {
		matches!(self.get_clear().data, NodeData::List(_))
	}
	
	/// Asks if the node is Map.
	pub fn is_map(&'a self) -> bool {
		matches!(self.get_clear().data, NodeData::Map(_))
	}
	
	/// Asks if the node is Tag.
	pub fn is_tag(&'a self) -> bool {
		use get_from::*;
		matches!(self.get_clear_advanced::<(TagData, TakeAnchorData, GetAnchorData)>().data, NodeData::Tag(_))
	}
	
	/// Asks if the node is File.
	pub fn is_file(&'a self) -> bool {
		use get_from::*;
		matches!(self.get_clear_advanced::<(FileData, TakeAnchorData, GetAnchorData)>().data, NodeData::File(_))
	}
	
	/// Asks if the node is TakeAnchor.
	pub fn is_take_anchor(&'a self) -> bool {
		use get_from::*;
		matches!(self.get_clear_advanced::<(TagData, FileData, GetAnchorData)>().data, NodeData::TakeAnchor(_))
	}
	
	/// Asks if the node is GetAnchor.
	pub fn is_get_anchor(&'a self) -> bool {
		use get_from::*;
		matches!(self.get_clear_advanced::<(TagData, FileData, TakeAnchorData)>().data, NodeData::GetAnchor(_))
	}
	
	/// Recursively gets a child node, excluding Tag, File, TakeAnchor and GetAnchor data.
	pub fn get_clear(&'a self) -> &'a Node {
		use get_from::*;
		get_from::<(TagData, FileData, TakeAnchorData, GetAnchorData)>(self)
	}
	
	/// Recursively gets a child node, excluding T.
	pub fn get_clear_advanced<T: get_from::GetFromStepType>(&'a self) -> &'a Node {
		use get_from::*;
		get_from::<T>(self)
	}
	
	/// Gets a child node if the node type is Tag, File, TakeAnchor or GetAnchor, otherwise the current node.
	pub fn get_clear_data(&'a self) -> Option<&'a Node> {
		use get_from::*;
		get_from_step::<(TagData, FileData, TakeAnchorData, GetAnchorData)>(&self.data)
	}
	
	/// Gets a child node if the node type is T, otherwise the current node.
	pub fn get_clear_data_advanced<T: get_from::GetFromStepType>(&'a self) -> Option<&'a Node> {
		use get_from::*;
		get_from_step::<T>(&self.data)
	}
	
	/// Gets the node under the Tag if the node type is with the Tag, otherwise the current node.
	pub fn get_clear_tag(&'a self) -> &'a Node {
		use get_from::*;
		get_from_step::<(FileData, TakeAnchorData, GetAnchorData)>(&self.data).unwrap_or(self)
	}
	
	/// Gets the node contained in the File, if the node type is a File, otherwise the current node.
	pub fn get_clear_file(&'a self) -> &'a Node {
		use get_from::*;
		get_from_step::<(TagData, TakeAnchorData, GetAnchorData)>(&self.data).unwrap_or(self)
	}
	
	/// Gets the node contained in the Anchor if the node type is TakeAnchor, otherwise the current node
	pub fn get_clear_take_anchor(&'a self) -> &'a Node {
		use get_from::*;
		get_from_step::<(TagData, TakeAnchorData, GetAnchorData)>(&self.data).unwrap_or(self)
	}
	
	/// Gets the node contained in the Anchor if the node type is GetAnchor, otherwise the current node
	pub fn get_clear_get_anchor(&'a self) -> &'a Node {
		use get_from::*;
		get_from_step::<(TagData, TakeAnchorData, GetAnchorData)>(&self.data).unwrap_or(self)
	}
	
	
	fn make_error<T: std::error::Error>(&'a self, error: T) -> marked::Error<T> {
		marked::Error::<T>::new(error, self.mark)
	}
	
	fn make_another_type_error(&'a self, requested_type: NodeType) -> marked::NodeAnotherTypeError {
		self.make_error(NodeAnotherTypeError::new(requested_type, self.get_type()))
	}
	
	/// Gets the tag.
	pub fn get_tag(&'a self) -> Result<&'a Tag, marked::NodeAnotherTypeError> {
		match &self.data {
			NodeData::Tag(i) => Ok(&i.tag),
			_ => Err(self.make_another_type_error(NodeType::Tag)),
		}
	}
	
	/// Gets the file path.
	pub fn get_file_path(&'a self) -> Result<&'a Path, marked::NodeAnotherTypeError> {
		match &self.data {
			NodeData::File(i) => Ok(&i.file_path),
			_ => Err(self.make_another_type_error(NodeType::File)),
		}
	}
	
	/// Gets the file anchor keeper.
	pub fn get_file_anchor_keeper(&'a self) -> Result<&'a AnchorKeeper, marked::NodeAnotherTypeError> {
		match &self.data {
			NodeData::File(i) => Ok(&i.anchor_keeper),
			_ => Err(self.make_another_type_error(NodeType::File)),
		}
	}
	
	/// Gets the take anchor name.
	pub fn get_take_anchor_name(&'a self) -> Result<&'a String, marked::NodeAnotherTypeError> {
		match &self.data {
			NodeData::TakeAnchor(i) => Ok(&i.name),
			_ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
		}
	}
	
	/// Gets the get anchor name.
	pub fn get_get_anchor_name(&'a self) -> Result<&'a String, marked::NodeAnotherTypeError> {
		match &self.data {
			NodeData::GetAnchor(i) => Ok(&i.name),
			_ => Err(self.make_another_type_error(NodeType::GetAnchor)),
		}
	}
	
	/// Gets the anchor name.
	pub fn get_anchor_name(&'a self) -> Result<&'a String, marked::NodeAnotherTypeError> {
		match &self.data {
			NodeData::TakeAnchor(i) => Ok(&i.name),
			NodeData::GetAnchor(i) => Ok(&i.name),
			_ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
		}
	}
	
	/// Gets the list size.
	pub fn get_list_size(&'a self) -> Result<usize, marked::NodeAnotherTypeError> {
		match &self.data {
			NodeData::List(i) => Ok(i.len()),
			_ => Err(self.make_another_type_error(NodeType::List)),
		}
	}
	
	/// Gets the map size.
	pub fn get_map_size(&'a self) -> Result<usize, marked::NodeAnotherTypeError> {
		match &self.data {
			NodeData::Map(i) => Ok(i.len()),
			_ => Err(self.make_another_type_error(NodeType::Map)),
		}
	}
	
	/// Gets the size.
	pub fn get_size(&'a self) -> Result<usize, marked::NodeAnotherTypeError> {
		match &self.data {
			NodeData::List(i) => Ok(i.len()),
			NodeData::Map(i) => Ok(i.len()),
			_ => Err(self.make_another_type_error(NodeType::Map))
		}
	}
	
	/// Gets the raw data.
	pub fn get_raw(&'a self) -> Result<&StringData, marked::NodeAnotherTypeError> {
		let clear = self.get_clear();
		get_typed_data_or_error!(clear, Raw)
	}
	
	/// Gets the string data.
	pub fn get_string(&'a self) -> Result<&RawData, marked::NodeAnotherTypeError> {
		let clear = self.get_clear();
		get_typed_data_or_error!(clear, String)
	}
	
	/// Gets the list data.
	pub fn get_list(&'a self) -> Result<&ListData, marked::NodeAnotherTypeError> {
		let clear = self.get_clear();
		get_typed_data_or_error!(clear, List)
	}
	
	/// Gets the map data.
	pub fn get_map(&'a self) -> Result<&MapData, marked::NodeAnotherTypeError> {
		let clear = self.get_clear();
		get_typed_data_or_error!(clear, Map)
	}
	
	/// Gets a node from the list by index.
	///
	/// # Arguments
	///
	/// * `index` Index.
	pub fn at_index(&'a self, index: usize) -> Result<&Node<'a>, marked::ListError> {
		match &self.data {
			NodeData::List(i) => i.get(index).ok_or_else(|| {
				marked::ListError::InvalidIndex(self.make_error(InvalidIndexError::new(index, i.len())))
			}),
			_ => Err(marked::ListError::NodeAnotherType(self.make_another_type_error(NodeType::List))),
		}
	}
	
	/// Gets a node from the map by key.
	///
	/// # Arguments
	///
	/// * `key` Key.
	pub fn at_key(&'a self, key: &str) -> Result<&Node<'a>, marked::MapError> {
		match &self.data {
			NodeData::Map(i) => i.get(key).ok_or_else(|| {
				marked::MapError::InvalidKey(self.make_error(InvalidKeyError::new(key.to_string())))
			}),
			_ => Err(marked::MapError::NodeAnotherType(self.make_another_type_error(NodeType::List))),
		}
	}
	
	///Gets the node by the index.
	///
	/// # Generic arguments
	///
	/// * `T` Index type.
	///
	/// # Arguments
	///
	/// * `index` Index.
	pub fn at<T: NodeIndex + 'a>(&'a self, index: T) -> Result<&Node<'a>, T::Error> {
		T::at(self, index)
	}
	
	/// Decodes the node into type T.
	///
	/// # Generic arguments
	///
	/// * `T` Value type.
	pub fn decode<T: Decode<'a> + 'a>(&'a self) -> Result<T, marked::FailedDecodeDataError> {
		T::decode(self).or_else(|e| {
			let error_box: Box<dyn std::error::Error> = match e {
				marked::DecodeError::NodeAnotherType(e) => Box::new(e),
				marked::DecodeError::InvalidIndex(e) => Box::new(e),
				marked::DecodeError::InvalidKey(e) => Box::new(e),
				marked::DecodeError::FailedDecodeData(e) => Box::new(e),
				marked::DecodeError::Failed(e) => e,
			};
			Err(self.make_error(FailedDecodeDataError::new::<T>(Some(error_box))))
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}