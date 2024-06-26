pub(crate) type Tag = String;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct TaggedNode {
    pub(crate) tag: Tag,
    pub(crate) node_index: usize,
}

impl TaggedNode {
    pub(crate) fn new(tag: Tag, node_index: usize) -> Self {
        Self { tag, node_index }
    }
}
