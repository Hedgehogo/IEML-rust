use super::super::name::Name;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct GetAnchorNode {
    pub name: Name<Box<str>>,
    pub node_index: usize,
}

impl GetAnchorNode {
    pub(crate) fn new(name: Name<Box<str>>, node_index: usize) -> Self {
        Self { name, node_index }
    }
}
