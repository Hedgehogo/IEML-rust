use std::collections::HashMap;
use crate::data::{
    node::{Node, MapIter},
    node_data::{Data, MarkedDataCell},
};
use crate::data::node_data::DataCell;

#[derive(Clone, Default, Eq)]
pub struct AnchorKeeper {
    anchors: HashMap<String, usize>,
    file_anchors: HashMap<String, usize>,
    parent: Option<usize>,
}

impl PartialEq for AnchorKeeper {
    fn eq(&self, other: &Self) -> bool {
        return self.anchors == other.anchors &&
            self.file_anchors == other.anchors;
    }
}

impl AnchorKeeper {
    pub fn new(parent: Option<usize>) -> Self {
        return Self {
            anchors: Default::default(),
            file_anchors: Default::default(),
            parent,
        };
    }
    
    pub fn add(&mut self, data: &mut Data, key: String, cell: MarkedDataCell) -> bool {
        let cell_index = data.len();
        data.insert(cell_index, cell);
        self.anchors.insert(key, cell_index).is_none()
    }
    
    pub fn add_to_file(&mut self, data: &mut Data, key: String, cell: MarkedDataCell) -> bool {
        let cell_index = data.len();
        data.insert(cell_index, cell);
        self.file_anchors.insert(key, cell_index).is_none()
    }
    
    pub fn get_anchors_iter<'a>(&'a self, data: &'a Data) -> MapIter<'a> {
        MapIter::new(data, self.anchors.iter())
    }
    
    pub fn get_file_anchors_iter<'a>(&'a self, data: &'a Data) -> MapIter<'a> {
        MapIter::new(data, self.file_anchors.iter())
    }
    
    pub fn get_parent<'a>(&'a self, data: &'a Data) -> Option<&'a AnchorKeeper> {
        self.parent.map(|i| match &data.get(&i).expect("Incorrect document structure, Cell does not exist.").cell {
            DataCell::File(i) => &i.anchor_keeper,
            _ => panic!("Incorrect document structure, the parent AnchorKeeper is not located in the file."),
        })
    }
    
    pub(crate) fn get_index<'a>(&'a self, data: &'a Data, key: &str) -> Option<usize> {
        match self.file_anchors.get(key) {
            Some(i) => Some(*i),
            None => match self.anchors.get(key) {
                Some(i) => Some(*i),
                None => self.get_parent(data).and_then(|i| i.get_index(data, key)),
            },
        }
    }
    
    pub fn get<'a>(&'a self, data: &'a Data, key: &str) -> Option<Node> {
        self.get_index(data, key).map(|i| Node::new(i, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn add_test() {
        let mut data = Data::default();
        let mut anchor_keeper = AnchorKeeper::new(None);
        assert!(anchor_keeper.add(&mut data, "key".to_string(), Default::default()));
        assert!(!anchor_keeper.add(&mut data, "key".to_string(), Default::default()));
    }
    
    #[test]
    fn add_to_file_test() {
        let mut data = Data::default();
        let mut anchor_keeper = AnchorKeeper::new(None);
        assert!(anchor_keeper.add_to_file(&mut data, "key".to_string(), Default::default()));
        assert!(!anchor_keeper.add_to_file(&mut data, "key".to_string(), Default::default()));
    }
}