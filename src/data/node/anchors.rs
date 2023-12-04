use super::super::cell::{Data, DataCell, FileCell};
use super::{iter::BasicMapIter, BasicNode};
use std::{collections::HashMap, error::Error, marker::PhantomData};

#[derive(Eq)]
pub struct Anchors<'a, E: Error + PartialEq + Eq> {
    cell: &'a FileCell,
    data: &'a Data,
    phantom: PhantomData<E>,
}

impl<'a, E: Error + PartialEq + Eq> PartialEq for Anchors<'a, E> {
    fn eq(&self, other: &Self) -> bool {
        return self.anchors() == other.anchors() && self.file_anchors() == other.file_anchors();
    }
}

impl<'a, E: Error + PartialEq + Eq> Anchors<'a, E> {
    pub(crate) fn new(cell: &'a FileCell, data: &'a Data) -> Self {
        Self {
            cell,
            data,
            phantom: Default::default(),
        }
    }

    fn anchors(&self) -> &'a HashMap<String, usize> {
        &self.cell.anchors
    }

    fn file_anchors(&self) -> &'a HashMap<String, usize> {
        &self.cell.file_anchors
    }

    pub fn parent(&self) -> Option<Anchors<'a, E>> {
        self.cell.parent.map(|i| match &self.data.get(i).cell {
            DataCell::File(i) => Self::new(i, self.data),
            _ => panic!("Incorrect document structure, the parent node is not a File."),
        })
    }

    pub fn anchors_iter(&self) -> BasicMapIter<'a, E> {
        BasicMapIter::new(self.data, self.anchors().iter())
    }

    pub fn file_anchors_iter(&self) -> BasicMapIter<'a, E> {
        BasicMapIter::new(self.data, self.file_anchors().iter())
    }

    pub(crate) fn get_index(&self, key: &str) -> Option<usize> {
        self.anchors().get(key).copied().or_else(|| {
            self.file_anchors()
                .get(key)
                .copied()
                .or_else(|| self.parent().and_then(|i| i.get_index(key)))
        })
    }

    pub fn get(&self, key: &str) -> Option<BasicNode<'a, E>> {
        self.get_index(key)
            .map(|i| BasicNode::new(self.data.get(i), self.data))
    }
}

impl<'a, E: Error + PartialEq + Eq> Clone for Anchors<'a, E> {
    fn clone(&self) -> Self {
        Self::new(self.cell, self.data)
    }
}

impl<'a, E: Error + PartialEq + Eq> Copy for Anchors<'a, E> {}
