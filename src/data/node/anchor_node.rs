use super::super::name::Name;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct AnchorNode {
    pub name: Name<Box<str>>,
    pub node_index: usize,
    pub creation: bool,
}

impl AnchorNode {
    pub(crate) fn new(name: Name<Box<str>>, node_index: usize, creation: bool) -> Self {
        Self { name, node_index, creation }
    }
}
