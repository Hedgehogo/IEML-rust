pub mod file_node;
pub mod get_anchor_node;
pub mod list_node;
pub mod map_node;
pub mod null_node;
pub mod raw_node;
pub mod string_node;
pub mod tagged_node;
pub mod take_anchor_node;

use super::{
    super::{
        cell::{Data, DataCell, MarkedDataCell},
        error::{marked, AnotherTypeError, FailedDeserializeError},
        mark::Mark,
        node_type::NodeType,
    },
    deserialize::Deserialize,
};
use std::{error::Error, fmt::Debug};
pub use {
    file_node::FileNode, get_anchor_node::GetAnchorNode, list_node::ListNode, map_node::MapNode,
    null_node::NullNode, raw_node::RawNode, string_node::StringNode, tagged_node::TaggedNode,
    take_anchor_node::TakeAnchorNode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node<'data> {
    Null(NullNode),
    Raw(RawNode<'data>),
    String(StringNode<'data>),
    List(ListNode<'data>),
    Map(MapNode<'data>),
    Tagged(TaggedNode<'data>),
    File(FileNode<'data>),
    TakeAnchor(TakeAnchorNode<'data>),
    GetAnchor(GetAnchorNode<'data>),
}

impl<'data> Node<'data> {
    pub(crate) fn new(cell: &'data MarkedDataCell, data: &'data Data) -> Self {
        match &cell.cell {
            DataCell::Null => Self::Null(NullNode::new(cell.mark)),
            DataCell::Raw(i) => Self::Raw(RawNode::new(cell.mark, i)),
            DataCell::String(i) => Self::String(StringNode::new(cell.mark, i)),
            DataCell::List(i) => Self::List(ListNode::new(cell.mark, i, data)),
            DataCell::Map(i) => Self::Map(MapNode::new(cell.mark, i, data)),
            DataCell::Tagged(i) => Self::Tagged(TaggedNode::new(cell.mark, i, data)),
            DataCell::File(i) => Self::File(FileNode::new(cell.mark, i, data)),
            DataCell::TakeAnchor(i) => Self::TakeAnchor(TakeAnchorNode::new(cell.mark, i, data)),
            DataCell::GetAnchor(i) => Self::GetAnchor(GetAnchorNode::new(cell.mark, i, data)),
        }
    }

    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        match self {
            Node::Null(i) => i.mark(),
            Node::Raw(i) => i.mark(),
            Node::String(i) => i.mark(),
            Node::List(i) => i.mark(),
            Node::Map(i) => i.mark(),
            Node::Tagged(i) => i.mark(),
            Node::File(i) => i.mark(),
            Node::TakeAnchor(i) => i.mark(),
            Node::GetAnchor(i) => i.mark(),
        }
    }

    /// Gets the node type.
    pub fn node_type(&self) -> NodeType {
        match self {
            Node::Null(_) => NodeType::Null,
            Node::Raw(_) => NodeType::Raw,
            Node::String(_) => NodeType::String,
            Node::List(_) => NodeType::List,
            Node::Map(_) => NodeType::Map,
            Node::Tagged(_) => NodeType::Tagged,
            Node::File(_) => NodeType::File,
            Node::TakeAnchor(_) => NodeType::TakeAnchor,
            Node::GetAnchor(_) => NodeType::GetAnchor,
        }
    }

    /// Returns whether the node is TakeAnchor.
    pub fn is_null(&self) -> bool {
        matches!(self.clear(), Self::Null(_))
    }

    /// Returns whether the node is Raw.
    pub fn is_raw(&self) -> bool {
        matches!(self.clear(), Self::Raw(_))
    }

    /// Returns whether the node is String.
    pub fn is_string(&self) -> bool {
        matches!(self.clear(), Self::String(_))
    }

    /// Returns whether the node is List.
    pub fn is_list(&self) -> bool {
        matches!(self.clear(), Self::List(_))
    }

    /// Returns whether the node is Map.
    pub fn is_map(&self) -> bool {
        matches!(self.clear(), Self::Map(_))
    }

    /// Returns whether the node is Tagged.
    pub fn is_tagged(&self) -> bool {
        use super::clear::*;
        matches!(
            self.clear_advanced::<(File, TakeAnchor, GetAnchor)>(),
            Self::Tagged(_)
        )
    }

    /// Returns whether the node is File.
    pub fn is_file(&self) -> bool {
        use super::clear::*;
        matches!(
            self.clear_advanced::<(Tagged, TakeAnchor, GetAnchor)>(),
            Self::File(_)
        )
    }

    /// Returns whether the node is TakeAnchor.
    pub fn is_take_anchor(&self) -> bool {
        use super::clear::*;
        matches!(
            self.clear_advanced::<(Tagged, File, GetAnchor)>(),
            Self::TakeAnchor(_)
        )
    }

    /// Returns whether the node is GetAnchor.
    pub fn is_get_anchor(&self) -> bool {
        use super::clear::*;
        matches!(
            self.clear_advanced::<(Tagged, File, TakeAnchor)>(),
            Self::GetAnchor(_)
        )
    }

    /// Gets a child node if the node type is Tagged.
    pub fn clear_step_tagged(&self) -> Option<Self> {
        match self {
            Self::Tagged(i) => Some(i.node()),
            _ => None,
        }
    }

    /// Gets a child node if the node type is File.
    pub fn clear_step_file(&self) -> Option<Self> {
        match self {
            Self::File(i) => Some(i.node()),
            _ => None,
        }
    }

    /// Gets a child node if the node type is TakeAnchor.
    pub fn clear_step_take_anchor(&self) -> Option<Self> {
        match self {
            Self::TakeAnchor(i) => Some(i.node()),
            _ => None,
        }
    }

    /// Gets a child node if the node type is GetAnchor.
    pub fn clear_step_get_anchor(&self) -> Option<Self> {
        match self {
            Self::GetAnchor(i) => Some(i.node()),
            _ => None,
        }
    }

    /// Gets a child node if the node type is Tagged, File, TakeAnchor or GetAnchor.
    pub fn clear_step(&self) -> Option<Self> {
        use super::clear::*;
        clear_step::<(Tagged, File, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets a child node if the node type is T.
    pub fn clear_step_advanced<T: super::clear::Clear<'data>>(&self) -> Option<Self> {
        use super::clear::*;
        clear_step::<T>(*self)
    }

    /// Recursively gets a child node, excluding Tagged, File, TakeAnchor and GetAnchor data.
    pub fn clear(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, File, TakeAnchor, GetAnchor)>(*self)
    }

    /// Recursively gets a child node, excluding T.
    pub fn clear_advanced<T: super::clear::Clear<'data>>(&self) -> Self {
        use super::clear::*;
        clear::<T>(*self)
    }

    /// Gets the node under the Tag if the node type is with the Tagged, otherwise the current node.
    pub fn clear_tag(&self) -> Self {
        use super::clear::*;
        clear::<(File, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets the node contained in the File, if the node type is a File, otherwise the current node.
    pub fn clear_file(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets the node contained in the Anchor if the node type is TakeAnchor, otherwise the current node
    pub fn clear_take_anchor(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, File, GetAnchor)>(*self)
    }

    /// Gets the node contained in the Anchor if the node type is GetAnchor, otherwise the current node
    pub fn clear_get_anchor(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, File, TakeAnchor)>(*self)
    }

    fn make_error<T: Error + PartialEq + Eq>(&self, error: T) -> marked::WithMarkError<T> {
        marked::WithMarkError::<T>::new(self.mark(), error)
    }

    fn make_another_type_error(&self, requested_type: NodeType) -> marked::AnotherTypeError {
        self.make_error(AnotherTypeError::new(requested_type, self.node_type()))
    }

    /// Gets the null data.
    pub fn null(&self) -> Result<NullNode, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear {
            Self::Null(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::Raw)),
        }
    }

    /// Gets the raw data.
    pub fn raw(&self) -> Result<RawNode<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear {
            Self::Raw(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::Raw)),
        }
    }

    /// Gets the string data.
    pub fn string(&self) -> Result<StringNode<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear {
            Self::String(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::String)),
        }
    }

    /// Gets the list node.
    pub fn list(&self) -> Result<ListNode<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear {
            Self::List(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::List)),
        }
    }

    /// Gets the map node.
    pub fn map(&self) -> Result<MapNode<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear {
            Self::Map(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::Map)),
        }
    }

    /// Gets the tagged node.
    pub fn tagged(&self) -> Result<TaggedNode<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, TakeAnchor, GetAnchor)>();
        match clear {
            Self::Tagged(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::Tagged)),
        }
    }

    /// Gets the file node.
    pub fn file(&self) -> Result<FileNode<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(Tagged, TakeAnchor, GetAnchor)>();
        match clear {
            Self::File(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::File)),
        }
    }

    /// Gets the take anchor node.
    pub fn take_anchor(&self) -> Result<TakeAnchorNode<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, Tagged, GetAnchor)>();
        match clear {
            Self::TakeAnchor(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
        }
    }

    /// Gets the get anchor node.
    pub fn get_anchor(&self) -> Result<GetAnchorNode<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, Tagged, TakeAnchor)>();
        match clear {
            Self::GetAnchor(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::GetAnchor)),
        }
    }

    /// Gets the anchor name.
    pub fn anchor_name(&self) -> Result<&str, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, Tagged)>();
        match &clear {
            Self::TakeAnchor(i) => Ok(i.name()),
            Self::GetAnchor(i) => Ok(i.name()),
            _ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
        }
    }

    /// Decodes the node into type T.
    ///
    /// # Generic arguments
    ///
    /// * `T` Value type.
    pub fn decode<E: Error + PartialEq + Eq, T: Deserialize<'data, E>>(
        &self,
    ) -> Result<T, marked::FailedDeserializeError<E>> {
        T::decode(*self).map_err(|e| self.make_error(FailedDeserializeError::new::<T>(Box::new(e))))
    }
}

#[cfg(test)]
mod tests;
