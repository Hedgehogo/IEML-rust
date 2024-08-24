//! Type definition [`DocumentView`]

use super::super::{
    super::{data::Data, mark::Mark, node::document_node::DocumentNode},
    analyse_anchors::AnalyseAnchors,
    anchors::Anchors,
    view::View,
};
use std::fmt;

/// Structure for reading Document node data.
#[derive(Clone, Eq)]
pub struct DocumentView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data DocumentNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> DocumentView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data DocumentNode,
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

    /// Gets the path.
    pub fn path(&self) -> &'data str {
        self.node.path.as_str()
    }

    /// Gets the view on the child node.
    pub fn view(&self) -> View<'data, A> {
        let node = self.data.get(self.node.node_index);
        View::new(node, self.data, self.anchor_analyser.clone())
    }

    /// Gets the structure for accessing anchors.
    pub fn anchors(&self) -> Anchors<'data, A> {
        let anchor_analyser = self.anchor_analyser.clone();
        Anchors::new(self.mark, self.node, self.data, anchor_analyser)
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for DocumentView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        self.anchors().document_anchors() == other.anchors().document_anchors()
            && self.view() == other.view()
    }
}

impl<'data, A: AnalyseAnchors<'data>> fmt::Debug for DocumentView<'data, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DocumentView {{ mark: {:?}, path: {:?}, anchors: {:?}, view: {:?} }}",
            self.mark,
            self.path(),
            self.anchors(),
            self.view()
        )
    }
}
