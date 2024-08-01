use super::map_node::MapNode;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct DocumentNode {
    pub(crate) path: PathBuf,
    pub(crate) node_index: usize,
    pub(crate) anchors: MapNode,
    pub(crate) document_anchors: MapNode,
    pub(crate) parent: Option<usize>,
}

impl DocumentNode {
    pub(crate) fn new(
        path: PathBuf,
        node_index: usize,
        anchors: MapNode,
        document_anchors: MapNode,
        parent: Option<usize>,
    ) -> Self {
        Self {
            path,
            node_index,
            anchors,
            document_anchors,
            parent,
        }
    }
}
