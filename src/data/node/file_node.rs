use super::{
    super::{
        cell::{file_cell::FileCell, Data},
        mark::Mark,
    },
    anchors::Anchors,
    Node,
};
use std::{
    fmt::{self, Debug, Formatter},
    path::Path,
};

#[derive(Clone, Copy, Eq)]
pub struct FileNode<'data> {
    mark: Mark,
    cell: &'data FileCell,
    data: &'data Data,
}

impl<'data> FileNode<'data> {
    pub(super) fn new(mark: Mark, cell: &'data FileCell, data: &'data Data) -> Self {
        Self { mark, cell, data }
    }

    pub(super) fn debug(&self, f: &mut Formatter<'_>) -> fmt::Result {
        FileCell::debug((self.cell, self.data), f)
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn path(&self) -> &'data Path {
        self.cell.path.as_path()
    }

    pub fn node(&self) -> Node<'data> {
        Node::new(self.data.get(self.cell.cell_index), self.data)
    }

    pub fn anchors(&self) -> Anchors<'data> {
        Anchors::new(self.mark, self.cell, self.data)
    }
}

impl<'data> PartialEq for FileNode<'data> {
    fn eq(&self, other: &Self) -> bool {
        FileCell::equal((self.cell, self.data), (other.cell, other.data))
    }
}

impl<'data> Debug for FileNode<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FileNode {{ mark: {:?}, cell: ", self.mark)?;
        FileCell::debug((&self.cell, &self.data), f)?;
        write!(f, " }}")
    }
}
