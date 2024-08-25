//! Type definition [`View`]

use super::{
    super::{
        data::Data,
        error::{marked, InvalidTypeError, InvalidValueError},
        mark::Mark,
        node::node::{MarkedNode, Node},
        node_type::NodeType,
    },
    analyse_anchors::AnalyseAnchors,
    deserialize::Deserialize,
};
use std::{any::type_name, error::Error, fmt::Debug};

pub use super::to_match::*;

/// Structure for reading node data.
#[derive(Clone, Copy, Eq)]
pub struct View<'data, A: AnalyseAnchors<'data> = ()> {
    node: &'data MarkedNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> View<'data, A> {
    pub(crate) fn new(node: &'data MarkedNode, data: &'data Data, anchor_analyser: A) -> Self {
        Self {
            node,
            data,
            anchor_analyser,
        }
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
            Node::Document(_) => NodeType::Document,
            Node::Anchor(_) => NodeType::Anchor,
        }
    }

    /// Gets a view that allows pattern matching.
    pub fn to_match(&self) -> ToMatchView<'data, A> {
        match &self.node.node {
            Node::Null => ToMatchView::Null(NullView::new(self.node.mark)),
            Node::Raw(i) => ToMatchView::Raw(RawView::new(self.node.mark, i)),
            Node::String(i) => ToMatchView::String(StringView::new(self.node.mark, i)),
            Node::List(i) => ToMatchView::List({
                let anchor_analyser = self.anchor_analyser.clone();
                ListView::new(self.node.mark, i, self.data, anchor_analyser)
            }),
            Node::Map(i) => ToMatchView::Map({
                let anchor_analyser = self.anchor_analyser.clone();
                MapView::new(self.node.mark, i, self.data, anchor_analyser)
            }),
            Node::Tagged(i) => ToMatchView::Tagged({
                let anchor_analyser = self.anchor_analyser.clone();
                TaggedView::new(self.node.mark, i, self.data, anchor_analyser)
            }),
            Node::Document(i) => ToMatchView::Document({
                let anchor_analyser = self.anchor_analyser.child(i.path.as_str());
                DocumentView::new(self.node.mark, i, self.data, anchor_analyser)
            }),
            Node::Anchor(i) => ToMatchView::Anchor({
                let anchor_analyser = self.anchor_analyser.clone();
                AnchorView::new(self.node.mark, i, self.data, anchor_analyser)
            }),
        }
    }

    /// Returns whether the node is Null.
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
        let clear = self.clear_advanced::<(Document, Anchor)>();
        matches!(clear.node.node, Node::Tagged(_))
    }

    /// Returns whether the node is Document.
    pub fn is_document(&self) -> bool {
        use super::clear::*;
        let clear = self.clear_advanced::<(Tagged, Anchor)>();
        matches!(clear.node.node, Node::Document(_))
    }

    /// Returns whether the node is Anchor.
    pub fn is_anchor(&self) -> bool {
        use super::clear::*;
        let clear = self.clear_advanced::<(Tagged, Document)>();
        matches!(clear.node.node, Node::Anchor(_))
    }

    /// Gets a child view if the node type is Tagged.
    pub fn clear_step_tagged(&self) -> Option<Self> {
        match &self.node.node {
            Node::Tagged(i) => Some(Self::new(
                self.data.get(i.node_index),
                self.data,
                self.anchor_analyser.clone(),
            )),
            _ => None,
        }
    }

    /// Gets a child view if the node type is Document.
    pub fn clear_step_document(&self) -> Option<Self> {
        match &self.node.node {
            Node::Document(i) => Some(Self::new(
                self.data.get(i.node_index),
                self.data,
                self.anchor_analyser.child(i.path.as_str()),
            )),
            _ => None,
        }
    }

    /// Gets a child view if the node type is Anchor.
    pub fn clear_step_anchor(&self) -> Option<Self> {
        match &self.node.node {
            Node::Anchor(i) => Some(Self::new(
                self.data.get(i.node_index),
                self.data,
                self.anchor_analyser.clone(),
            )),
            _ => None,
        }
    }

    /// Gets a child view if the node type is Tagged, Document or Anchor.
    pub fn clear_step(&self) -> Option<Self> {
        use super::clear::*;
        clear_step::<(Tagged, Document, Anchor), A>(self.clone())
    }

    /// Gets a child view if the node type is T.
    pub fn clear_step_advanced<T: super::clear::Clear<'data, A>>(&self) -> Option<Self> {
        use super::clear::*;
        clear_step::<T, A>(self.clone())
    }

    /// Recursively gets a child view, excluding Tagged, Document and Anchor data.
    pub fn clear(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, Document, Anchor), A>(self.clone())
    }

    /// Recursively gets a child view, excluding T.
    pub fn clear_advanced<T: super::clear::Clear<'data, A>>(&self) -> Self {
        use super::clear::*;
        clear::<T, A>(self.clone())
    }

    /// Recursively retrieves the nearest child view that is a Tag, if unsuccessful, returns the nearest view not containing a single child.
    pub fn clear_tag(&self) -> Self {
        use super::clear::*;
        clear::<(Document, Anchor), A>(self.clone())
    }

    /// Recursively retrieves the nearest child view that is a Document, if unsuccessful, returns the nearest view not containing a single child.
    pub fn clear_document(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, Anchor), A>(self.clone())
    }

    /// Recursively retrieves the nearest child view that is a Anchor, if unsuccessful, returns the nearest view not containing a single child.
    pub fn clear_anchor(&self) -> Self {
        use super::clear::*;
        clear::<(Tagged, Document), A>(self.clone())
    }

    fn make_error<T: Error + PartialEq + Eq>(&self, error: T) -> marked::MarkedError<T> {
        marked::MarkedError::<T>::new(self.mark(), error)
    }

    fn make_invalid_type_error(
        &self,
        expected_types: &'static [NodeType],
    ) -> marked::InvalidTypeError {
        self.make_error(InvalidTypeError::new(self.node_type(), expected_types))
    }

    /// Gets the null data.
    pub fn null(&self) -> Result<NullView, marked::InvalidTypeError> {
        let clear = self.clear();
        match &clear.node.node {
            Node::Null => Ok(NullView::new(clear.node.mark)),
            _ => {
                let expected = &[
                    NodeType::Null,
                    NodeType::Tagged,
                    NodeType::Anchor,
                    NodeType::Document,
                ];
                Err(self.make_invalid_type_error(expected))
            }
        }
    }

    /// Gets the raw data.
    pub fn raw(&self) -> Result<RawView<'data>, marked::InvalidTypeError> {
        let clear = self.clear();
        match &clear.node.node {
            Node::Raw(i) => Ok(RawView::new(clear.node.mark, i)),
            _ => {
                let expected = &[
                    NodeType::Raw,
                    NodeType::Tagged,
                    NodeType::Anchor,
                    NodeType::Document,
                ];
                Err(clear.make_invalid_type_error(expected))
            }
        }
    }

    /// Gets the string data.
    pub fn string(&self) -> Result<StringView<'data>, marked::InvalidTypeError> {
        let clear = self.clear();
        match &clear.node.node {
            Node::String(i) => Ok(StringView::new(clear.node.mark, i)),
            _ => {
                let expected = &[
                    NodeType::String,
                    NodeType::Tagged,
                    NodeType::Anchor,
                    NodeType::Document,
                ];
                Err(clear.make_invalid_type_error(expected))
            }
        }
    }

    /// Gets the list view.
    pub fn list(&self) -> Result<ListView<'data, A>, marked::InvalidTypeError> {
        let clear = self.clear();
        match &clear.node.node {
            Node::List(i) => Ok({
                let anchor_analyser = self.anchor_analyser.clone();
                ListView::new(clear.node.mark, i, clear.data, anchor_analyser)
            }),
            _ => {
                let expected = &[
                    NodeType::List,
                    NodeType::Tagged,
                    NodeType::Anchor,
                    NodeType::Document,
                ];
                Err(clear.make_invalid_type_error(expected))
            }
        }
    }

    /// Gets the map view.
    pub fn map(&self) -> Result<MapView<'data, A>, marked::InvalidTypeError> {
        let clear = self.clear();
        match &clear.node.node {
            Node::Map(i) => Ok({
                let anchor_analyser = self.anchor_analyser.clone();
                MapView::new(clear.node.mark, i, clear.data, anchor_analyser)
            }),
            _ => {
                let expected = &[
                    NodeType::Map,
                    NodeType::Tagged,
                    NodeType::Anchor,
                    NodeType::Document,
                ];
                Err(clear.make_invalid_type_error(expected))
            }
        }
    }

    /// Gets the tagged view.
    pub fn tagged(&self) -> Result<TaggedView<'data, A>, marked::InvalidTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(Document, Anchor)>();
        match &clear.node.node {
            Node::Tagged(i) => Ok({
                let anchor_analyser = self.anchor_analyser.clone();
                TaggedView::new(clear.node.mark, i, clear.data, anchor_analyser)
            }),
            _ => {
                let expected = &[NodeType::Tagged, NodeType::Anchor, NodeType::Document];
                Err(clear.make_invalid_type_error(expected))
            }
        }
    }

    /// Gets the document view.
    pub fn document(&self) -> Result<DocumentView<'data, A>, marked::InvalidTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(Tagged, Anchor)>();
        match &clear.node.node {
            Node::Document(i) => Ok({
                let anchor_analyser = self.anchor_analyser.child(i.path.as_str());
                DocumentView::new(clear.node.mark, i, clear.data, anchor_analyser)
            }),
            _ => {
                let expected = &[NodeType::Document, NodeType::Tagged, NodeType::Anchor];
                Err(clear.make_invalid_type_error(expected))
            }
        }
    }

    /// Gets the anchor view.
    pub fn anchor(&self) -> Result<AnchorView<'data, A>, marked::InvalidTypeError> {
        use super::clear::*;
        let clear = self.clear_advanced::<(Document, Tagged)>();
        match &clear.node.node {
            Node::Anchor(i) => Ok({
                let anchor_analyser = self.anchor_analyser.clone();
                AnchorView::new(clear.node.mark, i, clear.data, anchor_analyser)
            }),
            _ => {
                let expected = &[NodeType::Anchor, NodeType::Tagged, NodeType::Document];
                Err(clear.make_invalid_type_error(expected))
            }
        }
    }

    /// Decodes the view into type T.
    ///
    /// # Generic arguments
    ///
    /// * `T` Value type.
    pub fn decode<E: Error + PartialEq + Eq, T: Deserialize<'data, A, E>>(
        &self,
    ) -> Result<T, marked::InvalidValueError<E>> {
        T::deserialize(self.clone()).map_err(|e| {
            let expected = format!("value of type {:?}", type_name::<T>());
            self.make_error(InvalidValueError::new(expected, Some(Box::new(e))))
        })
    }

    /// Gets the id unique for the whole Data.
    pub fn anchor_analyser(self) -> A {
        self.anchor_analyser
    }

    /// Gets the id unique for the whole Data.
    pub fn id(&self) -> usize {
        self.node as *const _ as usize
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for View<'data, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_match())
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for View<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        self.to_match() == other.to_match()
    }
}

#[cfg(test)]
mod tests;
