use super::super::{
    cell::{Data, data_cell::{DataCell, MapCell, MarkedDataCell}},
    mark::Mark,
};
use std::path::{Path, PathBuf};

pub struct Maker<'a> {
    data: &'a mut Data,
    anchors: MapCell,
    path: PathBuf,
}

impl<'a> Maker<'a> {
    pub(super) fn new(data: &'a mut Data, path: PathBuf) -> Self {
        Self {
            data,
            anchors: Default::default(),
            path,
        }
    }

    pub(super) fn child<F: FnOnce(&mut Maker) -> R, R>(&mut self, f: F) -> R {
        let anchors = std::mem::take(&mut self.anchors);
        let result = f(self);
        self.anchors = anchors;
        result
    }

    pub(super) fn add(&mut self, mark: Mark, cell: DataCell) {
        self.data
            .data
            .insert(self.data.data.len(), MarkedDataCell::new(cell, mark));
    }

    pub(super) fn last(&self) -> usize {
        self.data.data.len() - 1
    }

    pub(super) fn add_anchor(&mut self, name: String, index: usize) -> Option<()> {
        self.anchors.data.insert(name, index).is_none().then_some(())
    }

    pub(super) fn anchors(&mut self) -> &mut MapCell {
        &mut self.anchors
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}
