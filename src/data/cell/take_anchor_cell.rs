use super::{super::node::Node, Data, DataCell};
use std::fmt::{self, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct TakeAnchorCell {
    pub name: String,
    pub cell_index: usize,
}

impl TakeAnchorCell {
    pub(crate) fn new(name: String, cell_index: usize) -> Self {
        Self { name, cell_index }
    }

    pub(crate) fn equal<'this, 'other>(
        this: (&'this Self, &'this Data),
        other: (&'other Self, &'other Data),
    ) -> bool {
        this.0.name == other.0.name
            && DataCell::equal(
                (&this.1.get(this.0.cell_index).cell, this.1),
                (&other.1.get(other.0.cell_index).cell, other.1),
            )
    }

    pub(crate) fn debug<'this>(
        this: (&'this Self, &'this Data),
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "name: {:?}, cell: {:?}",
            this.0.name,
            Node::new(this.1.get(this.0.cell_index), this.1)
        )
    }
}
