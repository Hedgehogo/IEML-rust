use super::{
    super::{
        data::Data,
        error::{marked, AnotherTypeError, FailedDeserializeError},
        mark::Mark,
        node::node::{MarkedNode, Node},
        node_type::NodeType,
    },
    deserialize::Deserialize,
};
use std::{error::Error, fmt::Debug};

pub use super::to_match::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct View<'data> {
    node: &'data MarkedNode,
    data: &'data Data,
}

impl<'data> View<'data> {
    pub(crate) fn new(node: &'data MarkedNode, data: &'data Data) -> Self {
        Self { node, data }
    }

    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        self.node.mark
    }

    /// Gets the node type.
    pub fn node_type(&self) -> NodeType {
        match &self.node.node {
            Node::Null => NodeType::Null,
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

    /// Gets a view that allows pattern matching.
    pub fn to_match(&self) -> ToMatchView {
        match &self.node.node {
            Node::Null => ToMatchView::Null(NullView::new(self.node.mark)),
            Node::Raw(i) => ToMatchView::Raw(RawView::new(self.node.mark, i)),
            Node::String(i) => ToMatchView::String(StringView::new(self.node.mark, i)),
            Node::List(i) => ToMatchView::List(ListView::new(self.node.mark, i, self.data)),
            Node::Map(i) => ToMatchView::Map(MapView::new(self.node.mark, i, self.data)),
            Node::Tagged(i) => ToMatchView::Tagged(TaggedView::new(self.node.mark, i, self.data)),
            Node::File(i) => ToMatchView::File(FileView::new(self.node.mark, i, self.data)),
            Node::TakeAnchor(i) => {
                ToMatchView::TakeAnchor(TakeAnchorView::new(self.node.mark, i, self.data))
            }
            Node::GetAnchor(i) => {
                ToMatchView::GetAnchor(GetAnchorView::new(self.node.mark, i, self.data))
            }
        }
    }

    /// Returns whether the node is TakeAnchor.
    pub fn is_null(&self) -> bool {
        matches!(self.clear().node.node, Node::Null)
    }

    /// Returns whether the node is Raw.
    pub fn is_raw(&self) -> bool {
        matches!(self.clear().node.node, Node::Raw(_))
    }

    /// Returns whether the node is String.
    pub fn is_string(&self) -> bool {
        matches!(self.clear().node.node, Node::String(_))
    }

    /// Returns whether the node is List.
    pub fn is_list(&self) -> bool {
        matches!(self.clear().node.node, Node::List(_))
    }

    /// Returns whether the node is Map.
    pub fn is_map(&self) -> bool {
        matches!(self.clear().node.node, Node::Map(_))
    }

    /// Returns whether the node is Tagged.
    pub fn is_tagged(&self) -> bool {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, TakeAnchor, GetAnchor)>();
        matches!(clear.node.node, Node::Tagged(_))
    }

    /// Returns whether the node is File.
    pub fn is_file(&self) -> bool {
        use super::clear::*;
        let clear = self.clear_advanced::<(Tagged, TakeAnchor, GetAnchor)>();
        matches!(clear.node.node, Node::File(_))
    }

    /// Returns whether the node is TakeAnchor.
    pub fn is_take_anchor(&self) -> bool {
        use super::clear::*;
        let clear = self.clear_advanced::<(Tagged, File, GetAnchor)>();
        matches!(clear.node.node, Node::TakeAnchor(_))
    }

    /// Returns whether the node is GetAnchor.
    pub fn is_get_anchor(&self) -> bool {
        use super::clear::*;
        let clear = self.clear_advanced::<(Tagged, File, TakeAnchor)>();
        matches!(clear.node.node, Node::GetAnchor(_))
    }

    /// Gets a child view if the node type is Tagged.
    pub fn clear_step_tagged(&self) -> Option<Self> {
        match &self.node.node {
            Node::Tagged(i) => Some(Self::new(self.data.get(i.node_index), self.data)),
            _ => None,
        }
    }

    /// Gets a child view if the node type is File.
    pub fn clear_step_file(&self) -> Option<Self> {
        match &self.node.node {
            Node::File(i) => Some(Self::new(self.data.get(i.node_index), self.data)),
            _ => None,
        }
    }

    /// Gets a child view if the node type is TakeAnchor.
    pub fn clear_step_take_anchor(&self) -> Option<Self> {
        match &self.node.node {
            Node::TakeAnchor(i) => Some(Self::new(self.data.get(i.node_index), self.data)),
            _ => None,
        }
    }

    /// Gets a child view if the node type is GetAnchor.
    pub fn clear_step_get_anchor(&self) -> Option<Self> {
        match &self.node.node {
            Node::GetAnchor(i) => Some(Self::new(self.data.get(i.node_index), self.data)),
            _ => None,
        }
    }

    /// Gets a child view if the node type is Tagged, File, TakeAnchor or GetAnchor.
    pub fn clear_step(&self) -> Option<Self> {
        use super::clear::*;
        clear_step::<(Tagged, File, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets a child view if the node type is T.
    pub fn clear_step_advanced<T: super::clear::Clear<'data>>(&self) -> Option<Self> {
        use super::clear::*;
        clear_step::<T>(*self)
    }

    /// Recursively gets a child view, excluding Tagged, File, TakeAnchor and GetAnchor data.
    pub fn clear(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, File, TakeAnchor, GetAnchor)>(*self)
    }

    /// Recursively gets a child view, excluding T.
    pub fn clear_advanced<T: super::clear::Clear<'data>>(&self) -> Self {
        use super::clear::*;
        clear::<T>(*self)
    }

    /// Gets the view under the Tag if the node type is with the Tagged, otherwise the current view.
    pub fn clear_tag(&self) -> Self {
        use super::clear::*;
        clear::<(File, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets the view contained in the File, if the node type is a File, otherwise the current view.
    pub fn clear_file(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets the view contained in the Anchor if the node type is TakeAnchor, otherwise the current view
    pub fn clear_take_anchor(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, File, GetAnchor)>(*self)
    }

    /// Gets the view contained in the Anchor if the node type is GetAnchor, otherwise the current view
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
    pub fn null(&self) -> Result<NullView, marked::AnotherTypeError> {
        let clear = self.clear();
        match &clear.node.node {
            Node::Null => Ok(NullView::new(clear.node.mark)),
            _ => Err(self.make_another_type_error(NodeType::Raw)),
        }
    }

    /// Gets the raw data.
    pub fn raw(&self) -> Result<RawView<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match &clear.node.node {
            Node::Raw(i) => Ok(RawView::new(clear.node.mark, i)),
            _ => Err(self.make_another_type_error(NodeType::Raw)),
        }
    }

    /// Gets the string data.
    pub fn string(&self) -> Result<StringView<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match &clear.node.node {
            Node::String(i) => Ok(StringView::new(clear.node.mark, i)),
            _ => Err(self.make_another_type_error(NodeType::String)),
        }
    }

    /// Gets the list view.
    pub fn list(&self) -> Result<ListView<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match &clear.node.node {
            Node::List(i) => Ok(ListView::new(clear.node.mark, i, clear.data)),
            _ => Err(self.make_another_type_error(NodeType::List)),
        }
    }

    /// Gets the map view.
    pub fn map(&self) -> Result<MapView<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match &clear.node.node {
            Node::Map(i) => Ok(MapView::new(clear.node.mark, i, clear.data)),
            _ => Err(self.make_another_type_error(NodeType::Map)),
        }
    }

    /// Gets the tagged view.
    pub fn tagged(&self) -> Result<TaggedView<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, TakeAnchor, GetAnchor)>();
        match &clear.node.node {
            Node::Tagged(i) => Ok(TaggedView::new(clear.node.mark, i, clear.data)),
            _ => Err(self.make_another_type_error(NodeType::Tagged)),
        }
    }

    /// Gets the file view.
    pub fn file(&self) -> Result<FileView<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(Tagged, TakeAnchor, GetAnchor)>();
        match &clear.node.node {
            Node::File(i) => Ok(FileView::new(clear.node.mark, i, clear.data)),
            _ => Err(self.make_another_type_error(NodeType::File)),
        }
    }

    /// Gets the take anchor view.
    pub fn take_anchor(&self) -> Result<TakeAnchorView<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, Tagged, GetAnchor)>();
        match &clear.node.node {
            Node::TakeAnchor(i) => Ok(TakeAnchorView::new(clear.node.mark, i, clear.data)),
            _ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
        }
    }

    /// Gets the get anchor view.
    pub fn get_anchor(&self) -> Result<GetAnchorView<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, Tagged, TakeAnchor)>();
        match &clear.node.node {
            Node::GetAnchor(i) => Ok(GetAnchorView::new(clear.node.mark, i, clear.data)),
            _ => Err(self.make_another_type_error(NodeType::GetAnchor)),
        }
    }

    /// Gets the anchor name.
    pub fn anchor_name(&self) -> Result<&str, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, Tagged)>();
        match &clear.node.node {
            Node::TakeAnchor(i) => Ok(i.name.as_str()),
            Node::GetAnchor(i) => Ok(i.name.as_str()),
            _ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
        }
    }

    /// Decodes the view into type T.
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

impl<'data> Debug for View<'data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_match())
    }
}

#[cfg(test)]
mod tests;
