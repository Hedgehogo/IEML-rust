use crate::data::cell::MarkedDataCell;

use super::{
    super::cell::{
        data_cell::{DataCell, FileCell},
        Data,
    },
    map_node::MapNode,
    Mark, Node,
};

#[derive(Clone, Copy, Eq)]
pub struct Anchors<'a> {
    mark: Mark,
    cell: &'a FileCell,
    data: &'a Data,
}

impl<'a> PartialEq for Anchors<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.anchors() == other.anchors() && self.file_anchors() == other.file_anchors();
    }
}

impl<'a> Anchors<'a> {
    pub(crate) fn new(mark: Mark, cell: &'a FileCell, data: &'a Data) -> Self {
        Self { mark, cell, data }
    }

    pub(crate) fn get_index(&self, key: &str) -> Option<usize> {
        self.cell.anchors.data.get(key).copied().or_else(|| {
            self.cell
                .file_anchors
                .data
                .get(key)
                .copied()
                .or_else(|| self.parent().and_then(|i| i.get_index(key)))
        })
    }

    pub fn parent(&self) -> Option<Anchors<'a>> {
        self.cell.parent.map(|i| match &self.data.get(i) {
            MarkedDataCell {
                mark,
                cell: DataCell::File(cell),
            } => Self::new(*mark, cell, self.data),
            _ => panic!("Incorrect document structure, the parent node is not a File."),
        })
    }

    pub fn anchors(&self) -> MapNode<'a> {
        MapNode::new(self.mark, &self.cell.anchors, self.data)
    }

    pub fn file_anchors(&self) -> MapNode<'a> {
        MapNode::new(self.mark, &self.cell.file_anchors, self.data)
    }

    pub fn get(&self, key: &str) -> Option<Node<'a>> {
        self.get_index(key)
            .map(|i| Node::new(self.data.get(i), self.data))
    }
}
