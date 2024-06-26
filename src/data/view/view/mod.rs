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

pub use super::type_view::{
    file_view::FileView, get_anchor_view::GetAnchorView, list_view::ListView, map_view::MapView,
    null_view::NullView, raw_view::RawView, string_view::StringView, tagged_view::TaggedView,
    take_anchor_view::TakeAnchorView,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View<'data> {
    Null(NullView),
    Raw(RawView<'data>),
    String(StringView<'data>),
    List(ListView<'data>),
    Map(MapView<'data>),
    Tagged(TaggedView<'data>),
    File(FileView<'data>),
    TakeAnchor(TakeAnchorView<'data>),
    GetAnchor(GetAnchorView<'data>),
}

impl<'data> View<'data> {
    pub(crate) fn new(node: &'data MarkedNode, data: &'data Data) -> Self {
        match &node.node {
            Node::Null => Self::Null(NullView::new(node.mark)),
            Node::Raw(i) => Self::Raw(RawView::new(node.mark, i)),
            Node::String(i) => Self::String(StringView::new(node.mark, i)),
            Node::List(i) => Self::List(ListView::new(node.mark, i, data)),
            Node::Map(i) => Self::Map(MapView::new(node.mark, i, data)),
            Node::Tagged(i) => Self::Tagged(TaggedView::new(node.mark, i, data)),
            Node::File(i) => Self::File(FileView::new(node.mark, i, data)),
            Node::TakeAnchor(i) => Self::TakeAnchor(TakeAnchorView::new(node.mark, i, data)),
            Node::GetAnchor(i) => Self::GetAnchor(GetAnchorView::new(node.mark, i, data)),
        }
    }

    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        match self {
            View::Null(i) => i.mark(),
            View::Raw(i) => i.mark(),
            View::String(i) => i.mark(),
            View::List(i) => i.mark(),
            View::Map(i) => i.mark(),
            View::Tagged(i) => i.mark(),
            View::File(i) => i.mark(),
            View::TakeAnchor(i) => i.mark(),
            View::GetAnchor(i) => i.mark(),
        }
    }

    /// Gets the view type.
    pub fn node_type(&self) -> NodeType {
        match self {
            View::Null(_) => NodeType::Null,
            View::Raw(_) => NodeType::Raw,
            View::String(_) => NodeType::String,
            View::List(_) => NodeType::List,
            View::Map(_) => NodeType::Map,
            View::Tagged(_) => NodeType::Tagged,
            View::File(_) => NodeType::File,
            View::TakeAnchor(_) => NodeType::TakeAnchor,
            View::GetAnchor(_) => NodeType::GetAnchor,
        }
    }

    /// Returns whether the view is TakeAnchor.
    pub fn is_null(&self) -> bool {
        matches!(self.clear(), Self::Null(_))
    }

    /// Returns whether the view is Raw.
    pub fn is_raw(&self) -> bool {
        matches!(self.clear(), Self::Raw(_))
    }

    /// Returns whether the view is String.
    pub fn is_string(&self) -> bool {
        matches!(self.clear(), Self::String(_))
    }

    /// Returns whether the view is List.
    pub fn is_list(&self) -> bool {
        matches!(self.clear(), Self::List(_))
    }

    /// Returns whether the view is Map.
    pub fn is_map(&self) -> bool {
        matches!(self.clear(), Self::Map(_))
    }

    /// Returns whether the view is Tagged.
    pub fn is_tagged(&self) -> bool {
        use super::clear::*;
        matches!(
            self.clear_advanced::<(File, TakeAnchor, GetAnchor)>(),
            Self::Tagged(_)
        )
    }

    /// Returns whether the view is File.
    pub fn is_file(&self) -> bool {
        use super::clear::*;
        matches!(
            self.clear_advanced::<(Tagged, TakeAnchor, GetAnchor)>(),
            Self::File(_)
        )
    }

    /// Returns whether the view is TakeAnchor.
    pub fn is_take_anchor(&self) -> bool {
        use super::clear::*;
        matches!(
            self.clear_advanced::<(Tagged, File, GetAnchor)>(),
            Self::TakeAnchor(_)
        )
    }

    /// Returns whether the view is GetAnchor.
    pub fn is_get_anchor(&self) -> bool {
        use super::clear::*;
        matches!(
            self.clear_advanced::<(Tagged, File, TakeAnchor)>(),
            Self::GetAnchor(_)
        )
    }

    /// Gets a child view if the view type is Tagged.
    pub fn clear_step_tagged(&self) -> Option<Self> {
        match self {
            Self::Tagged(i) => Some(i.view()),
            _ => None,
        }
    }

    /// Gets a child view if the view type is File.
    pub fn clear_step_file(&self) -> Option<Self> {
        match self {
            Self::File(i) => Some(i.view()),
            _ => None,
        }
    }

    /// Gets a child view if the view type is TakeAnchor.
    pub fn clear_step_take_anchor(&self) -> Option<Self> {
        match self {
            Self::TakeAnchor(i) => Some(i.view()),
            _ => None,
        }
    }

    /// Gets a child view if the view type is GetAnchor.
    pub fn clear_step_get_anchor(&self) -> Option<Self> {
        match self {
            Self::GetAnchor(i) => Some(i.view()),
            _ => None,
        }
    }

    /// Gets a child view if the view type is Tagged, File, TakeAnchor or GetAnchor.
    pub fn clear_step(&self) -> Option<Self> {
        use super::clear::*;
        clear_step::<(Tagged, File, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets a child view if the view type is T.
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

    /// Gets the view under the Tag if the view type is with the Tagged, otherwise the current view.
    pub fn clear_tag(&self) -> Self {
        use super::clear::*;
        clear::<(File, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets the view contained in the File, if the view type is a File, otherwise the current view.
    pub fn clear_file(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, TakeAnchor, GetAnchor)>(*self)
    }

    /// Gets the view contained in the Anchor if the view type is TakeAnchor, otherwise the current view
    pub fn clear_take_anchor(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, File, GetAnchor)>(*self)
    }

    /// Gets the view contained in the Anchor if the view type is GetAnchor, otherwise the current view
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
        match clear {
            Self::Null(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::Raw)),
        }
    }

    /// Gets the raw data.
    pub fn raw(&self) -> Result<RawView<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear {
            Self::Raw(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::Raw)),
        }
    }

    /// Gets the string data.
    pub fn string(&self) -> Result<StringView<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear {
            Self::String(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::String)),
        }
    }

    /// Gets the list view.
    pub fn list(&self) -> Result<ListView<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear {
            Self::List(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::List)),
        }
    }

    /// Gets the map view.
    pub fn map(&self) -> Result<MapView<'data>, marked::AnotherTypeError> {
        let clear = self.clear();
        match clear {
            Self::Map(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::Map)),
        }
    }

    /// Gets the tagged view.
    pub fn tagged(&self) -> Result<TaggedView<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, TakeAnchor, GetAnchor)>();
        match clear {
            Self::Tagged(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::Tagged)),
        }
    }

    /// Gets the file view.
    pub fn file(&self) -> Result<FileView<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(Tagged, TakeAnchor, GetAnchor)>();
        match clear {
            Self::File(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::File)),
        }
    }

    /// Gets the take anchor view.
    pub fn take_anchor(&self) -> Result<TakeAnchorView<'data>, marked::AnotherTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(File, Tagged, GetAnchor)>();
        match clear {
            Self::TakeAnchor(i) => Ok(i),
            _ => Err(self.make_another_type_error(NodeType::TakeAnchor)),
        }
    }

    /// Gets the get anchor view.
    pub fn get_anchor(&self) -> Result<GetAnchorView<'data>, marked::AnotherTypeError> {
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

#[cfg(test)]
mod tests;
