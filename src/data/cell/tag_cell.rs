use super::{super::node::Node, Data, DataCell};
use std::fmt::{self, Formatter};

pub(crate) type Tag = String;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct TagCell {
    pub(crate) tag: Tag,
    pub(crate) cell_index: usize,
}

impl TagCell {
    pub(crate) fn new(tag: Tag, cell_index: usize) -> Self {
        Self { tag, cell_index }
    }

    pub(crate) fn equal<'this, 'other>(
        this: (&'this Self, &'this Data),
        other: (&'other Self, &'other Data),
    ) -> bool {
        this.0.tag == other.0.tag
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
            "tag: {:?}, cell: {:?}",
            this.0.tag,
            Node::new(this.1.get(this.0.cell_index), this.1)
        )
    }
}
