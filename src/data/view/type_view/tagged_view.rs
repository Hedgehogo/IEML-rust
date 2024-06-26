use super::super::{
    super::{data::Data, mark::Mark, node::tag_node::TaggedNode},
    view::View,
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct TaggedView<'data> {
    mark: Mark,
    node: &'data TaggedNode,
    data: &'data Data,
}

impl<'data> TaggedView<'data> {
    pub(in super::super) fn new(mark: Mark, node: &'data TaggedNode, data: &'data Data) -> Self {
        Self { mark, node, data }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn tag(&self) -> &'data str {
        self.node.tag.as_str()
    }

    pub fn view(&self) -> View<'data> {
        View::new(self.data.get(self.node.node_index), self.data)
    }
}

impl<'data> PartialEq for TaggedView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.tag() == other.tag() && self.view() == other.view()
    }
}

impl<'data> Debug for TaggedView<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TaggedView {{ mark: {:?}, tag: {:?}, view: {:?} }}",
            self.mark,
            self.tag(),
            self.view()
        )
    }
}
