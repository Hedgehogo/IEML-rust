use super::{
    super::super::{
        cell::{get_anchor_cell::GetAnchorCell, Data},
        mark::Mark,
    },
    Node,
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct GetAnchorNode<'data> {
    mark: Mark,
    cell: &'data GetAnchorCell,
    data: &'data Data,
}

impl<'data> GetAnchorNode<'data> {
    pub(super) fn new(mark: Mark, cell: &'data GetAnchorCell, data: &'data Data) -> Self {
        Self { mark, cell, data }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn name(&self) -> &'data str {
        self.cell.name.as_str()
    }

    pub fn node(&self) -> Node<'data> {
        Node::new(self.data.get(self.cell.cell_index), self.data)
    }
}

impl<'data> PartialEq for GetAnchorNode<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl<'data> Debug for GetAnchorNode<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GetAnchorNode {{ mark: {:?}, name: {:?} }}",
            self.mark,
            self.name()
        )
    }
}
