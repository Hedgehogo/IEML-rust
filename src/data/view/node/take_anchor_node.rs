use super::{
    super::super::{
        cell::{take_anchor_cell::TakeAnchorCell, Data},
        mark::Mark,
    },
    Node,
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct TakeAnchorNode<'data> {
    mark: Mark,
    cell: &'data TakeAnchorCell,
    data: &'data Data,
}

impl<'data> TakeAnchorNode<'data> {
    pub(super) fn new(mark: Mark, cell: &'data TakeAnchorCell, data: &'data Data) -> Self {
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

impl<'data> PartialEq for TakeAnchorNode<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name() && self.node() == other.node()
    }
}

impl<'data> Debug for TakeAnchorNode<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TakeAnchorNode {{ mark: {:?}, name: {:?}, node: {:?} }}",
            self.mark,
            self.name(),
            self.node()
        )
    }
}
