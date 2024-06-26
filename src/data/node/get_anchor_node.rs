#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct GetAnchorNode {
    pub name: String,
    pub node_index: usize,
}

impl GetAnchorNode {
    pub(crate) fn new(name: String, node_index: usize) -> Self {
        Self { name, node_index }
    }
}
