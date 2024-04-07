pub(crate) mod data_cell;
pub(crate) mod file_cell;
pub(crate) mod get_anchor_cell;
pub(crate) mod list_cell;
pub(crate) mod map_cell;
pub(crate) mod tag_cell;
pub(crate) mod take_anchor_cell;

use super::view::node::Node;

pub(crate) use data_cell::{DataCell, MarkedDataCell};

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Data {
    pub(crate) data: Vec<MarkedDataCell>,
}

impl Data {
    #[allow(dead_code)]
    pub(crate) fn new<const N: usize>(data: [MarkedDataCell; N]) -> Self {
        Self {
            data: Vec::from(data),
        }
    }

    pub(crate) fn get(&self, index: usize) -> &MarkedDataCell {
        self.data
            .get(index)
            .expect("Incorrect document structure, Cell does not exist.")
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> &mut MarkedDataCell {
        self.data
            .get_mut(index)
            .expect("Incorrect document structure, Cell does not exist.")
    }

    pub fn node(&self) -> Node {
        Node::new(
            self.data
                .last()
                .expect("Incorrect document structure, Cell does not exist."),
            self,
        )
    }
}
