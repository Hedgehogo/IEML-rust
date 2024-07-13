use super::super::name::Name;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct TakeAnchorNode {
    pub name: Name,
    pub node_index: usize,
}

impl TakeAnchorNode {
    pub(crate) fn new(name: Name, node_index: usize) -> Self {
        Self { name, node_index }
    }
}
