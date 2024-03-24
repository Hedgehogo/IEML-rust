use super::{super::node::Node, Data, DataCell};
use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MapCell {
    pub(crate) data: HashMap<String, usize>,
}

impl MapCell {
    pub(crate) fn new(data: HashMap<String, usize>) -> Self {
        Self { data }
    }

    pub(crate) fn equal<'this, 'other>(
        this: (&'this Self, &'this Data),
        other: (&'other Self, &'other Data),
    ) -> bool {
        if this.0.data.len() == other.0.data.len() {
            this.0.data.iter().all(|(key, this_index)| {
                other.0.data.get(key).map_or(false, |other_index| {
                    DataCell::equal(
                        (&this.1.get(*this_index).cell, this.1),
                        (&other.1.get(*other_index).cell, other.1),
                    )
                })
            })
        } else {
            false
        }
    }

    pub(crate) fn debug<'this>(
        this: (&'this Self, &'this Data),
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{{")?;
        for i in this.0.data.iter() {
            write!(f, "{:?}: {:?}, ", i.0, Node::new(this.1.get(*i.1), this.1))?;
        }
        write!(f, "}}")
    }
}
