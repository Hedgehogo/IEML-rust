pub mod anchors;
mod clear;
pub mod decode;
mod index;
pub mod iter;

use super::{
    cell::{Data, DataCell, MarkedDataCell},
    error::{marked, AnotherTypeError, FailedDecodeError, InvalidIndexError, InvalidKeyError},
    mark::Mark,
    node_type::NodeType,
};
use anchors::Anchors;
use decode::Decode;
use index::NodeIndex;
use iter::{BasicListIter, BasicMapIter};
use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Formatter},
    marker::PhantomData,
    path::Path,
};

#[derive(Eq)]
pub struct BasicNode<'a, E: Error + PartialEq + Eq> {
    cell: &'a MarkedDataCell,
    data: &'a Data,
    phantom: PhantomData<E>,
}

pub type Node<'a> = BasicNode<'a, Infallible>;

impl<'a, E: Error + PartialEq + Eq> BasicNode<'a, E> {
    pub(crate) fn new(cell: &'a MarkedDataCell, data: &'a Data) -> Self {
        return Self {
            cell,
            data,
            phantom: Default::default(),
        };
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
        matches!(
            self.clear_advanced::<(File, TakeAnchor, GetAnchor)>()
                .cell(),
            DataCell::Tag(_)
        )
    }

    /// Returns whether the node is File.
    pub fn is_file(&self) -> bool {
        use clear::*;
        matches!(
            self.clear_advanced::<(Tag, TakeAnchor, GetAnchor)>().cell(),
            DataCell::File(_)
        )
    }

    /// Returns whether the node is TakeAnchor.
    pub fn is_take_anchor(&self) -> bool {
        use clear::*;
        matches!(
            self.clear_advanced::<(Tag, File, GetAnchor)>().cell(),
            DataCell::TakeAnchor(_)
        )
    }

    /// Returns whether the node is GetAnchor.
    pub fn is_get_anchor(&self) -> bool {
        use clear::*;
        matches!(
            self.clear_advanced::<(Tag, File, TakeAnchor)>().cell(),
            DataCell::GetAnchor(_)
        )
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
            Some(Self::new(
                self.data.get(take_anchor_cell.cell_index),
                self.data,
            ))
        } else {
            None
        }
    }

    /// Gets a child node if the node type is GetAnchor.
    pub fn clear_step_get_anchor(&self) -> Option<Self> {
        if let DataCell::GetAnchor(get_anchor_cell) = &self.cell() {
            Some(Self::new(
                self.data.get(get_anchor_cell.cell_index),
                self.data,
            ))
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
        clear::<E, (File, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets the node contained in the File, if the node type is a File, otherwise the current node.
    pub fn clear_file(&self) -> Self {
        use clear::*;
        clear::<E, (Tag, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets the node contained in the Anchor if the node type is TakeAnchor, otherwise the current node
    pub fn clear_take_anchor(&self) -> Self {
        use clear::*;
        clear::<E, (Tag, File, GetAnchor)>(*self)
    }

    /// Gets the node contained in the Anchor if the node type is GetAnchor, otherwise the current node
    pub fn clear_get_anchor(&self) -> Self {
        use clear::*;
        clear::<E, (Tag, File, TakeAnchor)>(*self)
    }

    fn make_error<T: Error + PartialEq + Eq>(&self, error: T) -> marked::WithMarkError<T> {
        marked::WithMarkError::<T>::new(self.mark(), error)
    }

    fn make_another_type_error(&self, requested_type: NodeType) -> marked::AnotherTypeError {
        self.make_error(AnotherTypeError::new(requested_type, self.node_type()))
    }

    /// Gets the raw data.
    pub fn raw(&self) -> Result<&'a str, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::Raw(i) => Ok(i.as_str()),
            _ => Err(clear.make_another_type_error(NodeType::Raw)),
        }
    }

    /// Gets the string data.
    pub fn string(&self) -> Result<&'a str, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::String(i) => Ok(i.as_str()),
            _ => Err(clear.make_another_type_error(NodeType::String)),
        }
    }

    /// Gets the list size.
    pub fn list_size(&self) -> Result<usize, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::List(i) => Ok(i.data.len()),
            _ => Err(clear.make_another_type_error(NodeType::List)),
        }
    }

    /// Gets the map size.
    pub fn map_size(&self) -> Result<usize, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::Map(i) => Ok(i.data.len()),
            _ => Err(clear.make_another_type_error(NodeType::Map)),
        }
    }

    /// Gets the size.
    pub fn size(&self) -> Result<usize, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::List(i) => Ok(i.data.len()),
            DataCell::Map(i) => Ok(i.data.len()),
            _ => Err(clear.make_another_type_error(NodeType::List)),
        }
    }

    /// Gets the tag.
    pub fn tag(&self) -> Result<&str, marked::AnotherTypeError> {
        let clear = self.clear_tag();
        match clear.cell() {
            DataCell::Tag(i) => Ok(&i.tag.as_str()),
            _ => Err(clear.make_another_type_error(NodeType::Tag)),
        }
    }

    /// Gets the file path.
    pub fn file_path(&self) -> Result<&Path, marked::AnotherTypeError> {
        let clear = self.clear_file();
        match clear.cell() {
            DataCell::File(i) => Ok(i.path.as_path()),
            _ => Err(clear.make_another_type_error(NodeType::File)),
        }
    }

    /// Gets the file anchor keeper.
    pub fn file_anchors(&self) -> Result<Anchors<E>, marked::AnotherTypeError> {
        let clear = self.clear_file();
        match clear.cell() {
            DataCell::File(i) => Ok(Anchors::new(i, clear.data)),
            _ => Err(clear.make_another_type_error(NodeType::File)),
        }
    }

    /// Gets the take anchor name.
    pub fn take_anchor_name(&self) -> Result<&str, marked::AnotherTypeError> {
        let clear = self.clear_take_anchor();
        match clear.cell() {
            DataCell::TakeAnchor(i) => Ok(&i.name.as_str()),
            _ => Err(clear.make_another_type_error(NodeType::TakeAnchor)),
        }
    }

    /// Gets the get anchor name.
    pub fn get_anchor_name(&self) -> Result<&str, marked::AnotherTypeError> {
        let clear = self.clear_get_anchor();
        match clear.cell() {
            DataCell::GetAnchor(i) => Ok(&i.name.as_str()),
            _ => Err(clear.make_another_type_error(NodeType::GetAnchor)),
        }
    }

    /// Gets the anchor name.
    pub fn anchor_name(&self) -> Result<&str, marked::AnotherTypeError> {
        let clear = self.clear_advanced::<(clear::File, clear::Tag)>();
        match clear.cell() {
            DataCell::TakeAnchor(i) => Ok(&i.name.as_str()),
            DataCell::GetAnchor(i) => Ok(&i.name.as_str()),
            _ => Err(clear.make_another_type_error(NodeType::TakeAnchor)),
        }
    }

    /// Gets the list data.
    pub fn list_iter(&self) -> Result<BasicListIter<'a, E>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::List(i) => Ok(BasicListIter::new(clear.data, i.data.iter())),
            _ => Err(clear.make_another_type_error(NodeType::List)),
        }
    }

    /// Gets the map data.
    pub fn map_iter(&self) -> Result<BasicMapIter<'a, E>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear.cell() {
            DataCell::Map(i) => Ok(BasicMapIter::new(clear.data, i.data.iter())),
            _ => Err(clear.make_another_type_error(NodeType::Map)),
        }
    }

    pub(crate) fn at_index(&self, index: usize) -> Result<Self, marked::ListError> {
        match self.cell() {
            DataCell::List(i) => match i.data.get(index) {
                Some(i) => Ok(Self::new(self.data.get(*i), self.data)),
                None => Err(marked::ListError::InvalidIndex(
                    self.make_error(InvalidIndexError::new(index, i.data.len())),
                )),
            },
            _ => Err(marked::ListError::NodeAnotherType(
                self.make_another_type_error(NodeType::List),
            )),
        }
    }

    pub(crate) fn at_key(&self, key: &str) -> Result<Self, marked::MapError> {
        match self.cell() {
            DataCell::Map(i) => match i.data.get(key) {
                Some(i) => Ok(Self::new(self.data.get(*i), self.data)),
                None => Err(marked::MapError::InvalidKey(
                    self.make_error(InvalidKeyError::new(key.to_string())),
                )),
            },
            _ => Err(marked::MapError::NodeAnotherType(
                self.make_another_type_error(NodeType::Map),
            )),
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
        T::at(self.clear(), index)
    }

    /// Decodes the node into type T.
    ///
    /// # Generic arguments
    ///
    /// * `T` Value type.
    pub fn decode<T: Decode<'a, E>>(&self) -> Result<T, marked::FailedDecodeError<E>> {
        T::decode(*self).map_err(|e| self.make_error(FailedDecodeError::new::<T>(Box::new(e))))
    }
}

impl<'a, E: Error + PartialEq + Eq> Clone for BasicNode<'a, E> {
    fn clone(&self) -> Self {
        Self::new(self.cell, self.data)
    }
}

impl<'a, E: Error + PartialEq + Eq> Copy for BasicNode<'a, E> {}

impl<'a, E: Error + PartialEq + Eq> Debug for BasicNode<'a, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ mark: {:?}, cell: ", self.mark())?;
        DataCell::debug((&self.cell(), &self.data), f)?;
        write!(f, " }}")
    }
}

impl<'a, E: Error + PartialEq + Eq> PartialEq for BasicNode<'a, E> {
    fn eq(&self, other: &Self) -> bool {
        DataCell::equal((self.cell(), self.data), (other.cell(), other.data))
    }
}

#[cfg(test)]
mod tests;
