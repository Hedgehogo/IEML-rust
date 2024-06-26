use super::{
    super::super::{
        mark::Mark,
        node::{take_anchor_node::TakeAnchorNode, Data},
    },
    View,
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct TakeAnchorView<'data> {
    mark: Mark,
    node: &'data TakeAnchorNode,
    data: &'data Data,
}

impl<'data> TakeAnchorView<'data> {
    pub(super) fn new(mark: Mark, node: &'data TakeAnchorNode, data: &'data Data) -> Self {
        Self { mark, node, data }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn name(&self) -> &'data str {
        self.node.name.as_str()
    }

    pub fn view(&self) -> View<'data> {
        View::new(self.data.get(self.node.node_index), self.data)
    }
}

impl<'data> PartialEq for TakeAnchorView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name() && self.view() == other.view()
    }
}

impl<'data> Debug for TakeAnchorView<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TakeAnchorView {{ mark: {:?}, name: {:?}, view: {:?} }}",
            self.mark,
            self.name(),
            self.view()
        )
    }
}
