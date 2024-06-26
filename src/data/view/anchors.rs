use super::{
    super::{
        data::Data,
        mark::Mark,
        node::node::{FileNode, MarkedNode, Node},
    },
    type_view::map_view::MapView,
    view::View,
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct Anchors<'data> {
    mark: Mark,
    node: &'data FileNode,
    data: &'data Data,
}

impl<'data> Anchors<'data> {
    pub(crate) fn new(mark: Mark, node: &'data FileNode, data: &'data Data) -> Self {
        Self { mark, node, data }
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

    pub fn parent(&self) -> Option<Anchors<'data>> {
        self.node.parent.map(|i| match &self.data.get(i) {
            MarkedNode {
                mark,
                node: Node::File(node),
            } => Self::new(*mark, node, self.data),
            _ => panic!("Incorrect document structure, the parent view is not a File."),
        })
    }

    pub fn anchors(&self) -> MapView<'data> {
        MapView::new(self.mark, &self.node.anchors, self.data)
    }

    pub fn file_anchors(&self) -> MapView<'data> {
        MapView::new(self.mark, &self.node.file_anchors, self.data)
    }

    pub fn get(&self, key: &str) -> Option<View<'data>> {
        self.get_index(key)
            .map(|i| View::new(self.data.get(i), self.data))
    }
}

impl<'data> PartialEq for Anchors<'data> {
    fn eq(&self, other: &Self) -> bool {
        return self.anchors() == other.anchors() && self.file_anchors() == other.file_anchors();
    }
}

impl<'data> Debug for Anchors<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Anchors {{ file_anchors {:?} }}", self.file_anchors())
    }
}
