use super::super::name::Name;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MapNode {
    pub(crate) data: HashMap<Name, usize>,
}

impl MapNode {
    pub(crate) fn new(data: HashMap<Name, usize>) -> Self {
        Self { data }
    }
}
