use super::{
    super::super::{
        cell::{tag_cell::TaggedCell, Data},
        mark::Mark,
    },
    Node,
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct TaggedNode<'data> {
    mark: Mark,
    cell: &'data TaggedCell,
    data: &'data Data,
}

impl<'data> TaggedNode<'data> {
    pub(super) fn new(mark: Mark, cell: &'data TaggedCell, data: &'data Data) -> Self {
        Self { mark, cell, data }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn tag(&self) -> &'data str {
        self.cell.tag.as_str()
    }

    pub fn node(&self) -> Node<'data> {
        Node::new(self.data.get(self.cell.cell_index), self.data)
    }
}

impl<'data> PartialEq for TaggedNode<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.tag() == other.tag() && self.node() == other.node()
    }
}

impl<'data> Debug for TaggedNode<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TaggedNode {{ mark: {:?}, tag: {:?}, node: {:?} }}",
            self.mark,
            self.tag(),
            self.node()
        )
    }
}
