//! Type definition [`AnchorView`]

use super::super::super::{data::Data, mark::Mark, name::Name, node::anchor_node::AnchorNode};
use super::super::{analyse_anchors::AnalyseAnchors, view::View};
use std::fmt;

/// Structure for reading Anchor node data.
#[derive(Clone, Eq)]
pub struct AnchorView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data AnchorNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> AnchorView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data AnchorNode,
        data: &'data Data,
        anchor_analyser: A,
    ) -> Self {
        Self {
            mark,
            node,
            data,
            anchor_analyser,
        }
    }

    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        self.mark
    }

    /// Gets the name.
    pub fn name(&self) -> Name<&'data str> {
        (&self.node.name).into()
    }

    /// Asks if the node is an anchor creation.
    pub fn is_creation(&self) -> bool {
        self.node.creation
    }

    /// Gets the view on the child node.
    pub fn view(&self) -> View<'data, A> {
        let node = self.data.get(self.node.node_index);
        View::new(node, self.data, self.anchor_analyser.clone())
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for AnchorView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        match (self.is_creation(), other.is_creation()) {
            (true, true) => self.name() == other.name() && self.view() == other.view(),
            (false, false) => self.name() == other.name(),
            _ => false,
        }
    }
}

impl<'data, A: AnalyseAnchors<'data>> fmt::Debug for AnchorView<'data, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_creation() {
            write!(
                f,
                "AnchorView {{ mark: {:?}, name: {:?}, view: {:?} }}",
                self.mark,
                self.name(),
                self.view()
            )
        } else {
            write!(
                f,
                "AnchorView {{ mark: {:?}, name: {:?} }}",
                self.mark,
                self.name()
            )
        }
    }
}
