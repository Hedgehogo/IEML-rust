#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct GetAnchorCell {
    pub name: String,
    pub cell_index: usize,
}

impl GetAnchorCell {
    pub(crate) fn new(name: String, cell_index: usize) -> Self {
        Self { name, cell_index }
    }
}
