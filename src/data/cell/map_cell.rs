use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MapCell {
    pub(crate) data: HashMap<String, usize>,
}

impl MapCell {
    pub(crate) fn new(data: HashMap<String, usize>) -> Self {
        Self { data }
    }
}
