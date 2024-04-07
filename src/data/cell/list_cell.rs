#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ListCell {
    pub(crate) data: Vec<usize>,
}

impl ListCell {
    pub(crate) fn new(data: Vec<usize>) -> Self {
        Self { data }
    }
}
