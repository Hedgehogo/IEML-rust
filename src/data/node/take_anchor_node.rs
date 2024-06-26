#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct TakeAnchorNode {
    pub name: String,
    pub node_index: usize,
}

impl TakeAnchorNode {
    pub(crate) fn new(name: String, node_index: usize) -> Self {
        Self { name, node_index }
    }
}
