mod get_from;
mod decode;

use std::{
    marker::PhantomData,
    path::Path,
    error::Error,
    convert::Infallible,
};
use super::{
    mark::Mark,
    node_type::NodeType,
    data_cell::{
        MarkedDataCell,
        DataCell,
        RawCell,
        StringCell,
        Tag,
        Data,
    },
    error::{
        AnotherTypeError,
        FailedDecodeError,
        InvalidIndexError,
        InvalidKeyError,
        marked,
    },
};
use decode::Decode;
use crate::anchor_keeper::AnchorKeeper;

pub trait NodeIndex<'a, E: Error + PartialEq + Eq> {
    type Error;
    
    fn at(node: BasicNode<'a, E>, index: Self) -> Result<BasicNode<'a, E>, Self::Error>;
}

impl<'a, E: Error + PartialEq + Eq> NodeIndex<'a, E> for usize {
    type Error = marked::ListError;
    
    fn at(node: BasicNode<'a, E>, index: Self) -> Result<BasicNode<'a, E>, Self::Error> {
        node.at_index(index)
    }
}

impl<'a, E: Error + PartialEq + Eq> NodeIndex<'a, E> for &str {
    type Error = marked::MapError;
    
    fn at(node: BasicNode<'a, E>, index: Self) -> Result<BasicNode<'a, E>, Self::Error> {
        node.at_key(index)
    }
}

pub struct BasicListIter<'a, E: Error + PartialEq + Eq> {
    data: &'a Data,
    iter: std::slice::Iter<'a, usize>,
    phantom: PhantomData<E>,
}

type ListIter<'a> = BasicListIter<'a, Infallible>;

impl<'a, E: Error + PartialEq + Eq> BasicListIter<'a, E> {
    pub fn new(data: &'a Data, iter: std::slice::Iter<'a, usize>) -> Self {
        Self { data, iter, phantom: Default::default() }
    }
}

impl<'a, E: Error + PartialEq + Eq> Clone for BasicListIter<'a, E> {
    fn clone(&self) -> Self {
        Self { data: self.data, iter: self.iter.clone(), phantom: Default::default() }
    }
}

impl<'a, E: Error + PartialEq + Eq> Iterator for BasicListIter<'a, E> {
    type Item = BasicNode<'a, E>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| BasicNode::new(*i, self.data))
    }
}

pub struct BasicMapIter<'a, E: Error + PartialEq + Eq> {
    data: &'a Data,
    iter: std::collections::hash_map::Iter<'a, String, usize>,
    phantom: PhantomData<E>,
}

type MapIter<'a> = BasicMapIter<'a, Infallible>;

impl<'a, E: Error + PartialEq + Eq> BasicMapIter<'a, E> {
    pub fn new(data: &'a Data, iter: std::collections::hash_map::Iter<'a, String, usize>) -> Self {
        Self { data, iter, phantom: Default::default() }
    }
}

impl<'a, E: Error + PartialEq + Eq> Clone for BasicMapIter<'a, E> {
    fn clone(&self) -> Self {
        Self { data: self.data, iter: self.iter.clone(), phantom: Default::default() }
    }
}

impl<'a, E: Error + PartialEq + Eq> Iterator for BasicMapIter<'a, E> {
    type Item = (&'a String, BasicNode<'a, E>);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| (i.0, BasicNode::new(*i.1, self.data)))
    }
}

#[derive(PartialEq, Eq)]
pub struct BasicNode<'a, E: Error + PartialEq + Eq> {
    cell_index: usize,
    data: &'a Data,
    phantom: PhantomData<E>,
}

type Node<'a> = BasicNode<'a, Infallible>;

impl<'a, E: Error + PartialEq + Eq> BasicNode<'a, E> {
    pub fn new(cell_index: usize, data: &'a Data) -> Self {
        return Self { cell_index, data, phantom: Default::default() };
    }
    
    fn cell(&self) -> &'a DataCell {
        &self.data.get(&self.cell_index).expect("Incorrect document structure, Cell does not exist.").cell
    }
    
    pub fn mark(&self) -> Mark {
        self.data.get(&self.cell_index).expect("Incorrect document structure, Cell does not exist.").mark
    }
    
    /// Gets the node type.
    pub fn node_type(&self) -> NodeType {
        self.cell().get_node_type()
    }
    
    /// Returns whether the node is TakeAnchor.
    pub fn is_null(&self) -> bool {
        matches!(self.clear().cell(), DataCell::Null)
    }
    
    /// Returns whether the node is Raw.
    pub fn is_raw(&self) -> bool {
        matches!(self.clear().cell(), DataCell::Raw(_))
    }
    
    /// Returns whether the node is String.
    pub fn is_string(&self) -> bool {
        matches!(self.clear().cell(), DataCell::String(_))
    }
    
    /// Returns whether the node is List.
    pub fn is_list(&self) -> bool {
        matches!(self.clear().cell(), DataCell::List(_))
    }
    
    /// Returns whether the node is Map.
    pub fn is_map(&self) -> bool {
        matches!(self.clear().cell(), DataCell::Map(_))
    }
    
    /// Returns whether the node is Tag.
    pub fn is_tag(&self) -> bool {
        use get_from::*;
        matches!(self.clear_advanced::<(TagData, TakeAnchorData, GetAnchorData)>().cell(), DataCell::Tag(_))
    }
    
    /// Returns whether the node is File.
    pub fn is_file(&self) -> bool {
        use get_from::*;
        matches!(self.clear_advanced::<(FileData, TakeAnchorData, GetAnchorData)>().cell(), DataCell::File(_))
    }
    
    /// Returns whether the node is TakeAnchor.
    pub fn is_take_anchor(&self) -> bool {
        use get_from::*;
        matches!(self.clear_advanced::<(TagData, FileData, GetAnchorData)>().cell(), DataCell::TakeAnchor(_))
    }
    
    /// Returns whether the node is GetAnchor.
    pub fn is_get_anchor(&self) -> bool {
        use get_from::*;
        matches!(self.clear_advanced::<(TagData, FileData, TakeAnchorData)>().cell(), DataCell::GetAnchor(_))
    }
    
    /// Recursively gets a child node, excluding Tag, File, TakeAnchor and GetAnchor data.
    pub fn clear(&self) -> Self<> {
        use get_from::*;
        get_from::<(TagData, FileData, TakeAnchorData, GetAnchorData), E>(*self)
    }
    
    /// Recursively gets a child node, excluding T.
    pub fn clear_advanced<T: get_from::GetFromStepType>(&self) -> Self {
        use get_from::*;
        get_from::<T, E>(*self)
    }
    
    /// Gets a child node if the node type is Tag, File, TakeAnchor or GetAnchor, otherwise the current node.
    pub fn clear_data(&self) -> Option<Self> {
        use get_from::*;
        get_from_step::<(TagData, FileData, TakeAnchorData, GetAnchorData), E>(*self)
    }
    
    /// Gets a child node if the node type is T, otherwise the current node.
    pub fn clear_data_advanced<T: get_from::GetFromStepType>(&self) -> Option<Self> {
        use get_from::*;
        get_from_step::<T, E>(*self)
    }
    
    /// Gets the node under the Tag if the node type is with the Tag, otherwise the current node.
    pub fn clear_tag(&self) -> Self {
        use get_from::*;
        get_from_step::<(FileData, TakeAnchorData, GetAnchorData), E>(*self).unwrap_or(*self)
    }
    
    /// Gets the node contained in the File, if the node type is a File, otherwise the current node.
    pub fn clear_file(&self) -> Self {
        use get_from::*;
        get_from_step::<(TagData, TakeAnchorData, GetAnchorData), E>(*self).unwrap_or(*self)
    }
    
    /// Gets the node contained in the Anchor if the node type is TakeAnchor, otherwise the current node
    pub fn clear_take_anchor(&self) -> Self {
        use get_from::*;
        get_from_step::<(TagData, TakeAnchorData, GetAnchorData), E>(*self).unwrap_or(*self)
    }
    
    /// Gets the node contained in the Anchor if the node type is GetAnchor, otherwise the current node
    pub fn clear_get_anchor(&self) -> Self {
        use get_from::*;
        get_from_step::<(TagData, TakeAnchorData, GetAnchorData), E>(*self).unwrap_or(*self)
    }
    
    fn make_error<T: Error + PartialEq + Eq>(&self, error: T) -> marked::WithMarkError<T> {
        marked::WithMarkError::<T>::new(error, self.mark())
    }
    
    fn make_another_type_error(&self, requested_type: NodeType) -> marked::AnotherTypeError {
        self.make_error(AnotherTypeError::new(requested_type, self.node_type()))
    }
    
    /// Gets the tag.
    pub fn tag(&self) -> Result<&Tag, marked::AnotherTypeError> {
        match self.cell() {
            DataCell::Tag(i) => Ok(&i.tag),
            _ => Err(self.make_another_type_error(NodeType::Tag)),
        }
    }
    
    /// Gets the file path.
    pub fn file_path(&self) -> Result<&Path, marked::AnotherTypeError> {
        match self.cell() {
            DataCell::File(i) => Ok(&i.path),
            _ => Err(self.make_another_type_error(NodeType::File)),
        }
    }
    
    /// Gets the file anchor keeper.
    pub fn file_anchor_keeper(&self) -> Result<&AnchorKeeper, marked::AnotherTypeError> {
        match self.cell() {
            DataCell::File(i) => Ok(&i.anchor_keeper),
            _ => Err(self.make_another_type_error(NodeType::File)),
        }
    }
    
    /// Gets the take anchor name.
    pub fn take_anchor_name(&self) -> Result<&String, marked::AnotherTypeError> {
        match self.cell() {
            DataCell::TakeAnchor(i) => Ok(&i.name),
            _ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
        }
    }
    
    /// Gets the get anchor name.
    pub fn get_anchor_name(&self) -> Result<&String, marked::AnotherTypeError> {
        match self.cell() {
            DataCell::GetAnchor(i) => Ok(&i.name),
            _ => Err(self.make_another_type_error(NodeType::GetAnchor)),
        }
    }
    
    /// Gets the anchor name.
    pub fn anchor_name(&self) -> Result<&String, marked::AnotherTypeError> {
        match self.cell() {
            DataCell::TakeAnchor(i) => Ok(&i.name),
            DataCell::GetAnchor(i) => Ok(&i.name),
            _ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
        }
    }
    
    /// Gets the list size.
    pub fn get_list_size(&self) -> Result<usize, marked::AnotherTypeError> {
        match self.cell() {
            DataCell::List(i) => Ok(i.len()),
            _ => Err(self.make_another_type_error(NodeType::List)),
        }
    }
    
    /// Gets the map size.
    pub fn get_map_size(&self) -> Result<usize, marked::AnotherTypeError> {
        match self.cell() {
            DataCell::Map(i) => Ok(i.len()),
            _ => Err(self.make_another_type_error(NodeType::Map)),
        }
    }
    
    /// Gets the size.
    pub fn get_size(&self) -> Result<usize, marked::AnotherTypeError> {
        match self.cell() {
            DataCell::List(i) => Ok(i.len()),
            DataCell::Map(i) => Ok(i.len()),
            _ => Err(self.make_another_type_error(NodeType::Map))
        }
    }
    
    /// Gets the raw data.
    pub fn get_raw(&self) -> Result<&'a RawCell, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::Raw(i) => Ok(i),
            _ => Err(clear.make_another_type_error(NodeType::Raw)),
        }
    }
    
    /// Gets the string data.
    pub fn get_string(&self) -> Result<&'a StringCell, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::String(i) => Ok(i),
            _ => Err(clear.make_another_type_error(NodeType::String)),
        }
    }
    
    /// Gets the list data.
    pub fn get_list_iter(&self) -> Result<BasicListIter<'a, E>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::List(i) => Ok(BasicListIter::new(self.data, i.into_iter())),
            _ => Err(clear.make_another_type_error(NodeType::List)),
        }
    }
    
    /// Gets the map data.
    pub fn get_map_iter(&self) -> Result<BasicMapIter<'a, E>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::Map(i) => Ok(BasicMapIter::new(self.data, i.into_iter())),
            _ => Err(clear.make_another_type_error(NodeType::List)),
        }
    }
    
    /// Gets a node from the list by index.
    ///
    /// # Arguments
    ///
    /// * `index` Index.
    pub fn at_index(&self, index: usize) -> Result<Self, marked::ListError> {
        match self.cell() {
            DataCell::List(i) => match i.get(index) {
                Some(i) => Ok(Self { cell_index: *i, data: self.data, phantom: Default::default() }),
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
    pub fn at_key(&self, key: &str) -> Result<Self, marked::MapError> {
        match self.cell() {
            DataCell::Map(i) => match i.get(key) {
                Some(i) => Ok(Self { cell_index: *i, data: self.data, phantom: Default::default() }),
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
    pub fn at<T: NodeIndex<'a, E>>(&self, index: T) -> Result<Self, T::Error> {
        T::at(*self, index)
    }
    
    /// Decodes the node into type T.
    ///
    /// # Generic arguments
    ///
    /// * `T` Value type.
    pub fn decode<T: Decode<'a, E>>(&self) -> Result<T, marked::FailedDecodeError<E>> {
        T::decode(*self).map_err(|e| {
            self.make_error(FailedDecodeError::new::<T>(Box::new(e)))
        })
    }
}

impl<'a, E: Error + PartialEq + Eq> Clone for BasicNode<'a, E> {
    fn clone(&self) -> Self {
        Self {
            cell_index: self.cell_index,
            data: self.data,
            phantom: Default::default(),
        }
    }
}

impl<'a, E: Error + PartialEq + Eq> Copy for BasicNode<'a, E> {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::data::data_cell::TagCell;
    use super::*;
    
    fn test_data() -> Data {
        Data::from([
            (0, MarkedDataCell { cell: DataCell::Null, mark: Default::default() }),
            (1, MarkedDataCell { cell: DataCell::Null, mark: Default::default() }),
            (2, MarkedDataCell { cell: DataCell::String("hello".into()), mark: Default::default() }),
            (3, MarkedDataCell { cell: DataCell::Raw("hello".into()), mark: Default::default() }),
            (4, MarkedDataCell { cell: DataCell::Tag(TagCell { cell_index: 0, tag: "tag".into() }), mark: Default::default() }),
        ])
    }
    
    #[test]
    fn test_list_iter_next() {
        let data = test_data();
        let list = vec![2_usize, 3];
        let mut list_iter = ListIter::new(&data, list.iter());
        
        let first = list_iter.next().unwrap();
        assert_eq!(first.node_type(), NodeType::String);
        assert_eq!(*first.get_string().unwrap(), "hello".to_string());
        
        let second = list_iter.next().unwrap();
        assert_eq!(second.node_type(), NodeType::Raw);
        assert_eq!(*second.get_raw().unwrap(), "hello".to_string());
        
        assert!(list_iter.next().is_none());
    }
    
    #[test]
    fn test_map_iter_next() {
        let data = test_data();
        let map = HashMap::<String, usize>::from([
            ("first".into(), 1),
            ("second".into(), 4),
        ]);
        let map_iter = MapIter::new(&data, map.iter());
        let mut collected_map = map_iter.collect::<Vec<(&String, Node)>>();
        collected_map.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(collected_map.len(), 2);
        
        let first = &collected_map[0];
        assert_eq!(*first.0, "first");
        assert_eq!(first.1.node_type(), NodeType::Null);
        
        let second = &collected_map[1];
        assert_eq!(*second.0, "second");
        assert_eq!(second.1.node_type(), NodeType::Tag);
        if let Ok(i) = second.1.get_raw() {
            assert_eq!(*i, "hello".to_string());
        }
    }
}