pub(crate) type Tag = String;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct TaggedCell {
    pub(crate) tag: Tag,
    pub(crate) cell_index: usize,
}

impl TaggedCell {
    pub(crate) fn new(tag: Tag, cell_index: usize) -> Self {
        Self { tag, cell_index }
    }
}
