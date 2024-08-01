use super::super::name::Name;
pub(crate) type Tag = Name<Box<str>>;

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
