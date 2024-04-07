use super::map_cell::MapCell;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FileCell {
    pub(crate) path: PathBuf,
    pub(crate) cell_index: usize,
    pub(crate) anchors: MapCell,
    pub(crate) file_anchors: MapCell,
    pub(crate) parent: Option<usize>,
}

impl FileCell {
    pub(crate) fn new(
        path: PathBuf,
        cell_index: usize,
        anchors: MapCell,
        file_anchors: MapCell,
        parent: Option<usize>,
    ) -> Self {
        Self {
            path,
            cell_index,
            anchors,
            file_anchors,
            parent,
        }
    }
}
