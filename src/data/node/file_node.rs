use super::map_node::MapNode;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FileNode {
    pub(crate) path: PathBuf,
    pub(crate) node_index: usize,
    pub(crate) anchors: MapNode,
    pub(crate) file_anchors: MapNode,
    pub(crate) parent: Option<usize>,
}

impl FileNode {
    pub(crate) fn new(
        path: PathBuf,
        node_index: usize,
        anchors: MapNode,
        file_anchors: MapNode,
        parent: Option<usize>,
    ) -> Self {
        Self {
            path,
            node_index,
            anchors,
            file_anchors,
            parent,
        }
    }
}
