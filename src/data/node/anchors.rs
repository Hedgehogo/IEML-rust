use std::{
    collections::HashMap,
    error::Error,
    marker::PhantomData
};
use super::{BasicNode, iter::BasicMapIter};
use super::super::data_cell::{Data, DataCell, FileCell};

#[derive(Eq)]
pub struct Anchors<'a, E: Error + PartialEq + Eq> {
    cell: &'a FileCell,
    data: &'a Data,
    phantom: PhantomData<E>,
}

impl<'a, E: Error + PartialEq + Eq> PartialEq for Anchors<'a, E> {
    fn eq(&self, other: &Self) -> bool {
        return self.anchors() == other.anchors() &&
            self.file_anchors() == other.file_anchors();
    }
}

impl<'a, E: Error + PartialEq + Eq> Anchors<'a, E> {
    pub(crate) fn new(cell: &'a FileCell, data: &'a Data) -> Self {
        Self { cell, data, phantom: Default::default() }
    }
    
    fn anchors(&self) -> &'a HashMap<String, usize> {
        &self.cell.anchors
    }
    
    fn file_anchors(&self) -> &'a HashMap<String, usize> {
        &self.cell.anchors
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
    
    pub fn get(&self, key: &str) -> Option<BasicNode<'a, E>> {
        match self.file_anchors().get(key) {
            Some(i) => Some(BasicNode::new(self.data.get(*i), self.data)),
            None => match self.anchors().get(key) {
                Some(i) => Some(BasicNode::new(self.data.get(*i), self.data)),
                None => self.parent().and_then(|i| i.get(key)),
            },
        }
    }
}

impl<'a, E: Error + PartialEq + Eq> Clone for Anchors<'a, E> {
    fn clone(&self) -> Self {
        Self::new(self.cell, self.data)
    }
}

impl<'a, E: Error + PartialEq + Eq> Copy for Anchors<'a, E> {}

#[cfg(test)]
mod tests {
    use super::*;
}