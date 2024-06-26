use crate::data::cell::MarkedDataCell;

use super::{
    super::{
        cell::{
            data_cell::{DataCell, FileCell},
            Data,
        },
        mark::Mark,
    },
    view::{map_view::MapView, View},
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct Anchors<'data> {
    mark: Mark,
    cell: &'data FileCell,
    data: &'data Data,
}

impl<'data> Anchors<'data> {
    pub(crate) fn new(mark: Mark, cell: &'data FileCell, data: &'data Data) -> Self {
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

    pub fn parent(&self) -> Option<Anchors<'data>> {
        self.cell.parent.map(|i| match &self.data.get(i) {
            MarkedDataCell {
                mark,
                cell: DataCell::File(cell),
            } => Self::new(*mark, cell, self.data),
            _ => panic!("Incorrect document structure, the parent view is not a File."),
        })
    }

    pub fn anchors(&self) -> MapView<'data> {
        MapView::new(self.mark, &self.cell.anchors, self.data)
    }

    pub fn file_anchors(&self) -> MapView<'data> {
        MapView::new(self.mark, &self.cell.file_anchors, self.data)
    }

    pub fn get(&self, key: &str) -> Option<View<'data>> {
        self.get_index(key)
            .map(|i| View::new(self.data.get(i), self.data))
    }
}

impl<'data> PartialEq for Anchors<'data> {
    fn eq(&self, other: &Self) -> bool {
        return self.anchors() == other.anchors() && self.file_anchors() == other.file_anchors();
    }
}

impl<'data> Debug for Anchors<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Anchors {{ file_anchors {:?} }}", self.file_anchors())
    }
}
