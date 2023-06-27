mod get_from;
mod decode;

use std::path::Path;
use super::{
    mark::Mark,
    node_type::NodeType,
    node_data::{
        MarkedDataCell,
        DataCell,
        RawCell,
        StringCell,
        Tag,
        Data
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
use crate::anchor_keeper::AnchorKeeper;

pub trait NodeIndex<'a> {
    type Error;
    
    fn at(node: Node<'a>, index: Self) -> Result<Node<'a>, Self::Error>;
}

impl<'a> NodeIndex<'a> for usize {
    type Error = marked::ListError;
    
    fn at(node: Node<'a>, index: Self) -> Result<Node<'a>, Self::Error> {
        node.at_index(index)
    }
}

impl<'a> NodeIndex<'a> for &str {
    type Error = marked::MapError;
    
    fn at(node: Node<'a>, index: Self) -> Result<Node<'a>, Self::Error> {
        node.at_key(index)
    }
}

#[derive(Clone)]
pub struct ListIter<'a> {
    data: &'a Data,
    iter: std::slice::Iter<'a, usize>,
}

impl<'a> ListIter<'a> {
    pub fn new(data: &'a Data, iter: std::slice::Iter<'a, usize>) -> Self {
        Self{data, iter}
    }
}

impl<'a> Iterator for ListIter<'a> {
    type Item = Node<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
       self.iter.next().map(|i| Node::new(*i, self.data))
    }
}

#[derive(Clone)]
pub struct MapIter<'a> {
    data: &'a Data,
    iter: std::collections::hash_map::Iter<'a, String, usize>,
}

impl<'a> MapIter<'a> {
    pub fn new(data: &'a Data, iter: std::collections::hash_map::Iter<'a, String, usize>) -> Self {
        Self{data, iter}
    }
}

impl<'a> Iterator for MapIter<'a> {
    type Item = (&'a String, Node<'a>);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| (i.0, Node::new(*i.1, self.data)))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Node<'a> {
    cell_index: usize,
    data: &'a Data,
}

//type ListIter<'a> = std::iter::Map<std::slice::Iter<'a, usize>, fn(&usize) -> Node<'a>>;
//type MapIter<'a> = std::iter::Map<std::collections::hash_map::IntoIter<String, usize>, fn((&String, &usize)) -> (&'a String, Node<'a>)>;

impl<'a> Node<'a> {
    pub fn new(cell_index: usize, data: &'a Data) -> Node {
        return Self{ cell_index, data };
    }
    
    /// Gets the marked data cell.
    fn get_marked_cell(&self) -> &'a MarkedDataCell {
        &self.data.get(&self.cell_index).expect("Incorrect document structure, Cell does not exist.")
    }
    
    fn get_cell(&self) -> &'a DataCell {
        &self.data.get(&self.cell_index).expect("Incorrect document structure, Cell does not exist.").cell
    }
    
    pub fn get_mark(&self) -> Mark {
        self.data.get(&self.cell_index).expect("Incorrect document structure, Cell does not exist.").mark
    }
    
    /// Gets the node type.
    pub fn get_type(&self) -> NodeType {
        self.get_cell().get_node_type()
    }
    
    /// Asks if the node is Null.
    pub fn is_null(&self) -> bool {
        matches!(self.get_clear().get_cell(), DataCell::Null)
    }
    
    /// Asks if the node is Raw.
    pub fn is_raw(&self) -> bool {
        matches!(self.get_clear().get_cell(), DataCell::Raw(_))
    }
    
    /// Asks if the node is String.
    pub fn is_string(&self) -> bool {
        matches!(self.get_clear().get_cell(), DataCell::String(_))
    }
    
    /// Asks if the node is List.
    pub fn is_list(&self) -> bool {
        matches!(self.get_clear().get_cell(), DataCell::List(_))
    }
    
    /// Asks if the node is Map.
    pub fn is_map(&self) -> bool {
        matches!(self.get_clear().get_cell(), DataCell::Map(_))
    }
    
    /// Asks if the node is Tag.
    pub fn is_tag(&self) -> bool {
        use get_from::*;
        matches!(self.get_clear_advanced::<(TagData, TakeAnchorData, GetAnchorData)>().get_cell(), DataCell::Tag(_))
    }
    
    /// Asks if the node is File.
    pub fn is_file(&self) -> bool {
        use get_from::*;
        matches!(self.get_clear_advanced::<(FileData, TakeAnchorData, GetAnchorData)>().get_cell(), DataCell::File(_))
    }
    
    /// Asks if the node is TakeAnchor.
    pub fn is_take_anchor(&self) -> bool {
        use get_from::*;
        matches!(self.get_clear_advanced::<(TagData, FileData, GetAnchorData)>().get_cell(), DataCell::TakeAnchor(_))
    }
    
    /// Asks if the node is GetAnchor.
    pub fn is_get_anchor(&self) -> bool {
        use get_from::*;
        matches!(self.get_clear_advanced::<(TagData, FileData, TakeAnchorData)>().get_cell(), DataCell::GetAnchor(_))
    }
    
    /// Recursively gets a child node, excluding Tag, File, TakeAnchor and GetAnchor data.
    pub fn get_clear(&self) -> Node<'a> {
        use get_from::*;
        get_from::<(TagData, FileData, TakeAnchorData, GetAnchorData)>(*self)
    }
    
    /// Recursively gets a child node, excluding T.
    pub fn get_clear_advanced<T: get_from::GetFromStepType>(&self) -> Node<'a> {
        use get_from::*;
        get_from::<T>(*self)
    }
    
    /// Gets a child node if the node type is Tag, File, TakeAnchor or GetAnchor, otherwise the current node.
    pub fn get_clear_data(&self) -> Option<Node<'a>> {
        use get_from::*;
        get_from_step::<(TagData, FileData, TakeAnchorData, GetAnchorData)>(*self)
    }
    
    /// Gets a child node if the node type is T, otherwise the current node.
    pub fn get_clear_data_advanced<T: get_from::GetFromStepType>(&self) -> Option<Node<'a>> {
        use get_from::*;
        get_from_step::<T>(*self)
    }
    
    /// Gets the node under the Tag if the node type is with the Tag, otherwise the current node.
    pub fn get_clear_tag(&self) -> Node<'a> {
        use get_from::*;
        get_from_step::<(FileData, TakeAnchorData, GetAnchorData)>(*self).unwrap_or(*self)
    }
    
    /// Gets the node contained in the File, if the node type is a File, otherwise the current node.
    pub fn get_clear_file(&self) -> Node<'a> {
        use get_from::*;
        get_from_step::<(TagData, TakeAnchorData, GetAnchorData)>(*self).unwrap_or(*self)
    }
    
    /// Gets the node contained in the Anchor if the node type is TakeAnchor, otherwise the current node
    pub fn get_clear_take_anchor(&self) -> Node<'a> {
        use get_from::*;
        get_from_step::<(TagData, TakeAnchorData, GetAnchorData)>(*self).unwrap_or(*self)
    }
    
    /// Gets the node contained in the Anchor if the node type is GetAnchor, otherwise the current node
    pub fn get_clear_get_anchor(&self) -> Node<'a> {
        use get_from::*;
        get_from_step::<(TagData, TakeAnchorData, GetAnchorData)>(*self).unwrap_or(*self)
    }
    
    fn make_error<T: std::error::Error>(&self, error: T) -> marked::Error<T> {
        marked::Error::<T>::new(error, self.get_mark())
    }
    
    fn make_another_type_error(&self, requested_type: NodeType) -> marked::NodeAnotherTypeError {
        self.make_error(NodeAnotherTypeError::new(requested_type, self.get_type()))
    }
    
    /// Gets the tag.
    pub fn get_tag(&self) -> Result<&Tag, marked::NodeAnotherTypeError> {
        match self.get_cell() {
            DataCell::Tag(i) => Ok(&i.tag),
            _ => Err(self.make_another_type_error(NodeType::Tag)),
        }
    }
    
    /// Gets the file path.
    pub fn get_file_path(&self) -> Result<&Path, marked::NodeAnotherTypeError> {
        match self.get_cell() {
            DataCell::File(i) => Ok(&i.path),
            _ => Err(self.make_another_type_error(NodeType::File)),
        }
    }
    
    /// Gets the file anchor keeper.
    pub fn get_file_anchor_keeper(&self) -> Result<&AnchorKeeper, marked::NodeAnotherTypeError> {
        match self.get_cell() {
            DataCell::File(i) => Ok(&i.anchor_keeper),
            _ => Err(self.make_another_type_error(NodeType::File)),
        }
    }
    
    /// Gets the take anchor name.
    pub fn get_take_anchor_name(&self) -> Result<&String, marked::NodeAnotherTypeError> {
        match self.get_cell() {
            DataCell::TakeAnchor(i) => Ok(&i.name),
            _ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
        }
    }
    
    /// Gets the get anchor name.
    pub fn get_get_anchor_name(&self) -> Result<&String, marked::NodeAnotherTypeError> {
        match self.get_cell() {
            DataCell::GetAnchor(i) => Ok(&i.name),
            _ => Err(self.make_another_type_error(NodeType::GetAnchor)),
        }
    }
    
    /// Gets the anchor name.
    pub fn get_anchor_name(&self) -> Result<&String, marked::NodeAnotherTypeError> {
        match self.get_cell() {
            DataCell::TakeAnchor(i) => Ok(&i.name),
            DataCell::GetAnchor(i) => Ok(&i.name),
            _ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
        }
    }
    
    /// Gets the list size.
    pub fn get_list_size(&self) -> Result<usize, marked::NodeAnotherTypeError> {
        match self.get_cell() {
            DataCell::List(i) => Ok(i.len()),
            _ => Err(self.make_another_type_error(NodeType::List)),
        }
    }
    
    /// Gets the map size.
    pub fn get_map_size(&self) -> Result<usize, marked::NodeAnotherTypeError> {
        match self.get_cell() {
            DataCell::Map(i) => Ok(i.len()),
            _ => Err(self.make_another_type_error(NodeType::Map)),
        }
    }
    
    /// Gets the size.
    pub fn get_size(&self) -> Result<usize, marked::NodeAnotherTypeError> {
        match self.get_cell() {
            DataCell::List(i) => Ok(i.len()),
            DataCell::Map(i) => Ok(i.len()),
            _ => Err(self.make_another_type_error(NodeType::Map))
        }
    }
    
    /// Gets the raw data.
    pub fn get_raw(&self) -> Result<&'a StringCell, marked::NodeAnotherTypeError> {
        let clear = self.get_clear();
        match clear.get_cell() {
            DataCell::Raw(i) => Ok(i),
            _ => Err(clear.make_another_type_error(NodeType::Raw)),
        }
    }
    
    /// Gets the string data.
    pub fn get_string(&self) -> Result<&'a RawCell, marked::NodeAnotherTypeError> {
        let clear = self.get_clear();
        match clear.get_cell() {
            DataCell::String(i) => Ok(i),
            _ => Err(clear.make_another_type_error(NodeType::String)),
        }
    }
    
    /// Gets the list data.
    pub fn get_list_iter(&self) -> Result<ListIter<'a>, marked::NodeAnotherTypeError> {
        let clear = self.get_clear();
        match clear.get_cell() {
            DataCell::List(i) => Ok(ListIter::new(self.data, i.into_iter())),
            _ => Err(clear.make_another_type_error(NodeType::List)),
        }
    }
    
    /// Gets the map data.
    pub fn get_map_iter(&self) -> Result<MapIter<'a>, marked::NodeAnotherTypeError> {
        let clear = self.get_clear();
        match clear.get_cell() {
            DataCell::Map(i) => Ok(MapIter::new(self.data, i.into_iter())),
            _ => Err(clear.make_another_type_error(NodeType::List)),
        }
    }
    
    /// Gets a node from the list by index.
    ///
    /// # Arguments
    ///
    /// * `index` Index.
    pub fn at_index(&self, index: usize) -> Result<Node<'a>, marked::ListError> {
        match self.get_cell() {
            DataCell::List(i) => match i.get(index) {
                Some(i) => Ok(Node { cell_index: *i, data: self.data }),
                None => Err(marked::ListError::InvalidIndex(self.make_error(InvalidIndexError::new(index, i.len()))))
            }
            _ => Err(marked::ListError::NodeAnotherType(self.make_another_type_error(NodeType::List))),
        }
    }
    
    /// Gets a node from the map by key.
    ///
    /// # Arguments
    ///
    /// * `key` Key.
    pub fn at_key(&self, key: &str) -> Result<Node<'a>, marked::MapError> {
        match self.get_cell() {
            DataCell::Map(i) => match i.get(key) {
                Some(i) => Ok(Node { cell_index: *i, data: self.data }),
                None => Err(marked::MapError::InvalidKey(self.make_error(InvalidKeyError::new(key.to_string()))))
            }
            _ => Err(marked::MapError::NodeAnotherType(self.make_another_type_error(NodeType::Map))),
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
    pub fn at<T: NodeIndex<'a>>(&self, index: T) -> Result<Node<'a>, T::Error> {
        T::at(*self, index)
    }
    
    /// Decodes the node into type T.
    ///
    /// # Generic arguments
    ///
    /// * `T` Value type.
    pub fn decode<T: Decode<'a>>(&self) -> Result<T, marked::FailedDecodeDataError> {
        T::decode(*self).or_else(|e| {
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