use super::super::{
    super::{data::Data, mark::Mark, node::get_anchor_node::GetAnchorNode},
    view::View,
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct GetAnchorView<'data> {
    mark: Mark,
    node: &'data GetAnchorNode,
    data: &'data Data,
}

impl<'data> GetAnchorView<'data> {
    pub(in super::super) fn new(mark: Mark, node: &'data GetAnchorNode, data: &'data Data) -> Self {
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

impl<'data> PartialEq for GetAnchorView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl<'data> Debug for GetAnchorView<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GetAnchorView {{ mark: {:?}, name: {:?} }}",
            self.mark,
            self.name()
        )
    }
}
