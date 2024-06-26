use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MapNode {
    pub(crate) data: HashMap<String, usize>,
}

impl MapNode {
    pub(crate) fn new(data: HashMap<String, usize>) -> Self {
        Self { data }
    }
}
