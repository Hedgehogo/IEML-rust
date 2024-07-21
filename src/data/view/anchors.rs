use super::{
    super::{
        data::Data,
        mark::Mark,
        node::node::{FileNode, MarkedNode, Node},
    },
    analyse_anchors::AnalyseAnchors,
    type_view::map_view::MapView,
    view::View,
};
use std::fmt::{self, Debug, Formatter};

/// Structure for reading file-level anchor data
#[derive(Clone, Copy, Eq)]
pub struct Anchors<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data FileNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> Anchors<'data, A> {
    pub(crate) fn new(
        mark: Mark,
        node: &'data FileNode,
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

    pub(crate) fn get_index(&self, key: &str) -> Option<usize> {
        self.node.anchors.data.get(key).copied().or_else(|| {
            self.node
                .file_anchors
                .data
                .get(key)
                .copied()
                .or_else(|| self.parent().and_then(|i| i.get_index(key)))
        })
    }

    /// Gets the same structure for the parent file, if it exists
    pub fn parent(&self) -> Option<Anchors<'data, A>> {
        self.node
            .parent
            .zip(self.anchor_analyser.parent())
            .map(|(i, parent)| match &self.data.get(i) {
                MarkedNode {
                    mark,
                    node: Node::File(node),
                } => Self::new(*mark, node, self.data, parent),
                _ => panic!("Incorrect document structure, the parent view is not a File."),
            })
    }

    /// Gets a map of anchors created directly inside the file
    pub fn anchors(&self) -> MapView<'data, A> {
        let map_node = &self.node.anchors;
        let anchor_analyser = self.anchor_analyser.clone();
        MapView::new(self.mark, map_node, self.data, anchor_analyser)
    }

    /// Gets the map of anchors created as external anchors passed to the file at the time of its loading.
    pub fn file_anchors(&self) -> MapView<'data, A> {
        let map_node = &self.node.file_anchors;
        let anchor_analyser = self.anchor_analyser.clone();
        MapView::new(self.mark, map_node, self.data, anchor_analyser)
    }

    /// Gets the view to the child anchor node for the given file using the same algorithm defined by the IEML standard.
    ///
    /// # Arguments
    /// * `key` Key of the requested anchor.
    pub fn get(&self, key: &str) -> Option<View<'data, A>> {
        self.get_index(key).map(|i| {
            let node = self.data.get(i);
            let anchor_analyser = self.anchor_analyser.clone();
            View::new(node, self.data, anchor_analyser)
        })
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for Anchors<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        return self.anchors() == other.anchors() && self.file_anchors() == other.file_anchors();
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for Anchors<'data, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Anchors {{ file_anchors: {:?} }}", self.file_anchors())
    }
}
