mod clear;
mod index;
pub mod decode;
pub mod iter;
pub mod anchors;

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
        Tag,
        RawCell,
        StringCell,
        DataCell,
        MarkedDataCell,
        Data,
        equal_data
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
use index::NodeIndex;
use iter::{BasicListIter, BasicMapIter};
use anchors::Anchors;

#[derive(Eq)]
pub struct BasicNode<'a, E: Error + PartialEq + Eq> {
    cell: &'a MarkedDataCell,
    data: &'a Data,
    phantom: PhantomData<E>,
}

type Node<'a> = BasicNode<'a, Infallible>;

impl<'a, E: Error + PartialEq + Eq> BasicNode<'a, E> {
    pub(crate) fn new(cell: &'a MarkedDataCell, data: &'a Data) -> Self {
        return Self { cell, data, phantom: Default::default() };
    }
    
    pub(crate) fn cell(&self) -> &'a DataCell {
        &self.cell.cell
    }
    
    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        self.cell.mark
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
        use clear::*;
        matches!(self.clear_advanced::<(Tag, TakeAnchor, GetAnchor)>().cell(), DataCell::Tag(_))
    }
    
    /// Returns whether the node is File.
    pub fn is_file(&self) -> bool {
        use clear::*;
        matches!(self.clear_advanced::<(File, TakeAnchor, GetAnchor)>().cell(), DataCell::File(_))
    }
    
    /// Returns whether the node is TakeAnchor.
    pub fn is_take_anchor(&self) -> bool {
        use clear::*;
        matches!(self.clear_advanced::<(Tag, File, GetAnchor)>().cell(), DataCell::TakeAnchor(_))
    }
    
    /// Returns whether the node is GetAnchor.
    pub fn is_get_anchor(&self) -> bool {
        use clear::*;
        matches!(self.clear_advanced::<(Tag, File, TakeAnchor)>().cell(), DataCell::GetAnchor(_))
    }
    
    /// Gets a child node if the node type is Tag.
    pub fn clear_step_tag(&self) -> Option<Self> {
        if let DataCell::Tag(tag_cell) = &self.cell() {
            Some(Self::new(self.data.get(tag_cell.cell_index), self.data))
        } else {
            None
        }
    }
    
    /// Gets a child node if the node type is File.
    pub fn clear_step_file(&self) -> Option<Self> {
        if let DataCell::File(file_cell) = &self.cell() {
            Some(Self::new(self.data.get(file_cell.cell_index), self.data))
        } else {
            None
        }
    }
    
    /// Gets a child node if the node type is TakeAnchor.
    pub fn clear_step_take_anchor(&self) -> Option<Self> {
        if let DataCell::TakeAnchor(take_anchor_cell) = &self.cell() {
            Some(Self::new(self.data.get(take_anchor_cell.cell_index), self.data))
        } else {
            None
        }
    }
    
    /// Gets a child node if the node type is GetAnchor.
    pub fn clear_step_get_anchor(&self) -> Option<Self> {
        if let DataCell::GetAnchor(get_anchor_cell) = &self.cell() {
            Some(Self::new(self.data.get(get_anchor_cell.cell_index), self.data))
        } else {
            None
        }
    }
    
    /// Gets a child node if the node type is Tag, File, TakeAnchor or GetAnchor.
    pub fn clear_step(&self) -> Option<Self> {
        use clear::*;
        clear_step::<E, (Tag, File, TakeAnchor, GetAnchor)>(*self)
    }
    
    /// Gets a child node if the node type is T.
    pub fn clear_step_advanced<T: clear::ClearStepType<E>>(&self) -> Option<Self> {
        use clear::*;
        clear_step::<E, T>(*self)
    }
    
    /// Recursively gets a child node, excluding Tag, File, TakeAnchor and GetAnchor data.
    pub fn clear(&self) -> Self {
        use clear::*;
        clear::<E, (Tag, File, TakeAnchor, GetAnchor)>(*self)
    }
    
    /// Recursively gets a child node, excluding T.
    pub fn clear_advanced<T: clear::ClearStepType<E>>(&self) -> Self {
        use clear::*;
        clear::<E, T>(*self)
    }
    
    /// Gets the node under the Tag if the node type is with the Tag, otherwise the current node.
    pub fn clear_tag(&self) -> Self {
        use clear::*;
        clear_step::<E, (File, TakeAnchor, GetAnchor)>(*self).unwrap_or(*self)
    }
    
    /// Gets the node contained in the File, if the node type is a File, otherwise the current node.
    pub fn clear_file(&self) -> Self {
        use clear::*;
        clear_step::<E, (Tag, TakeAnchor, GetAnchor)>(*self).unwrap_or(*self)
    }
    
    /// Gets the node contained in the Anchor if the node type is TakeAnchor, otherwise the current node
    pub fn clear_take_anchor(&self) -> Self {
        use clear::*;
        clear_step::<E, (Tag, File, GetAnchor)>(*self).unwrap_or(*self)
    }
    
    /// Gets the node contained in the Anchor if the node type is GetAnchor, otherwise the current node
    pub fn clear_get_anchor(&self) -> Self {
        use clear::*;
        clear_step::<E, (Tag, TakeAnchor, GetAnchor)>(*self).unwrap_or(*self)
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
    pub fn file_anchor_keeper(&self) -> Result<Anchors<E>, marked::AnotherTypeError> {
        match self.cell() {
            DataCell::File(i) => Ok(Anchors::new(i, self.data)),
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
    
    pub(crate) fn at_index(&self, index: usize) -> Result<Self, marked::ListError> {
        match self.cell() {
            DataCell::List(i) => match i.get(index) {
                Some(i) => Ok(Self::new(
                    self.data.get(*i),
                    self.data,
                )),
                None => Err(marked::ListError::InvalidIndex(self.make_error(InvalidIndexError::new(index, i.len()))))
            }
            _ => Err(marked::ListError::NodeAnotherType(self.make_another_type_error(NodeType::List))),
        }
    }
    
    pub(crate) fn at_key(&self, key: &str) -> Result<Self, marked::MapError> {
        match self.cell() {
            DataCell::Map(i) => match i.get(key) {
                Some(i) => Ok(Self::new(
                    self.data.get(*i),
                    self.data,
                )),
                None => Err(marked::MapError::InvalidKey(self.make_error(InvalidKeyError::new(key.to_string()))))
            }
            _ => Err(marked::MapError::NodeAnotherType(self.make_another_type_error(NodeType::Map))),
        }
    }
    
    /// Gets the node by the index.
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
        Self::new(self.cell, self.data)
    }
}

impl<'a, E: Error + PartialEq + Eq> Copy for BasicNode<'a, E> {}

impl <'a, E: Error + PartialEq + Eq> PartialEq for BasicNode<'a, E> {
    fn eq(&self, other: &Self) -> bool {
        equal_data(self.cell(), self.data, other.cell(), other.data)
    }
}