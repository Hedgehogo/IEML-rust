use std::fmt::{self, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct GetAnchorCell {
    pub name: String,
    pub cell_index: usize,
}

impl GetAnchorCell {
    pub(crate) fn new(name: String, cell_index: usize) -> Self {
        Self { name, cell_index }
    }

    pub(crate) fn equal(&self, other: &Self) -> bool {
        self.name == other.name
    }

    pub(crate) fn debug<'this>(&'this self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "name: {:?}", self.name)
    }
}
