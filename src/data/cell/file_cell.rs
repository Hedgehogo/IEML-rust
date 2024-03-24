use super::{super::node::Node, map_cell::MapCell, Data};
use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    path::PathBuf,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FileCell {
    pub(crate) path: PathBuf,
    pub(crate) cell_index: usize,
    pub(crate) anchors: HashMap<String, usize>,
    pub(crate) file_anchors: MapCell,
    pub(crate) parent: Option<usize>,
}

impl FileCell {
    pub(crate) fn new(
        path: PathBuf,
        cell_index: usize,
        anchors: HashMap<String, usize>,
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

    pub(crate) fn equal<'this, 'other>(
        this: (&'this Self, &'this Data),
        other: (&'other Self, &'other Data),
    ) -> bool {
        this.0.path == other.0.path
            && this.0.anchors.len() == other.0.anchors.len()
            && this.1.get(this.0.cell_index) == other.1.get(other.0.cell_index)
            && MapCell::equal(
                (&this.0.file_anchors, this.1),
                (&other.0.file_anchors, other.1),
            )
    }

    pub(crate) fn debug<'this>(
        this: (&'this Self, &'this Data),
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "file-path: {:?}, anchors: ", this.0.path)?;
        MapCell::debug((&this.0.file_anchors, this.1), f)?;
        write!(
            f,
            ", cell: {:?}",
            Node::new(this.1.get(this.0.cell_index), this.1)
        )
    }
}
