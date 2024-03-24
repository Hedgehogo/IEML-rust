use super::{super::node::Node, Data, DataCell};
use std::fmt::{self, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ListCell {
    pub(crate) data: Vec<usize>,
}

impl ListCell {
    pub(crate) fn new(data: Vec<usize>) -> Self {
        Self { data }
    }

    pub(crate) fn equal<'this, 'other>(
        this: (&'this Self, &'this Data),
        other: (&'other Self, &'other Data),
    ) -> bool {
        if this.0.data.len() == other.0.data.len() {
            this.0
                .data
                .iter()
                .zip(other.0.data.iter())
                .all(|(this_index, other_index)| {
                    DataCell::equal(
                        (&this.1.get(*this_index).cell, this.1),
                        (&other.1.get(*other_index).cell, other.1),
                    )
                })
        } else {
            false
        }
    }

    pub(crate) fn debug<'this>(
        this: (&'this Self, &'this Data),
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "[")?;
        for i in this.0.data.iter() {
            write!(f, "{:?}, ", Node::new(this.1.get(*i), this.1))?;
        }
        write!(f, "]")
    }
}
