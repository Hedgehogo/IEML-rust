use super::super::{
    cell::{Data, DataCell, MarkedDataCell},
    mark::Mark,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Maker<'a> {
    data: &'a mut Data,
    anchors: HashMap<String, usize>,
    path: PathBuf,
}

impl<'a> Maker<'a> {
    pub(crate) fn new(data: &'a mut Data, path: PathBuf) -> Self {
        Self {
            data,
            anchors: Default::default(),
            path,
        }
    }

    pub(crate) fn child<F: FnOnce(&mut Maker) -> R, R>(&mut self, f: F) -> R {
        let anchors = std::mem::take(&mut self.anchors);
        let result = f(self);
        self.anchors = anchors;
        result
    }

    pub(crate) fn add(&mut self, mark: Mark, cell: DataCell) {
        self.data
            .data
            .insert(self.data.data.len(), MarkedDataCell { cell, mark });
    }

    pub(crate) fn last(&self) -> usize {
        self.data.data.len() - 1
    }

    pub(crate) fn add_anchor(&mut self, name: String, index: usize) -> Option<()> {
        self.anchors.insert(name, index).is_none().then_some(())
    }

    pub(crate) fn anchors(&mut self) -> &mut HashMap<String, usize> {
        &mut self.anchors
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}
