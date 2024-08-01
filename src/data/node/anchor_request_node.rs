use super::super::name::Name;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct AnchorRequestNode {
    pub name: Name<Box<str>>,
    pub node_index: usize,
}

impl AnchorRequestNode {
    pub(crate) fn new(name: Name<Box<str>>, node_index: usize) -> Self {
        Self { name, node_index }
    }
}
