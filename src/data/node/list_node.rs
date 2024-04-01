use super::{
    super::{
        cell::{data_cell::ListCell, Data},
        error::{marked, InvalidIndexError},
        mark::Mark,
    },
    Node,
};
use std::{
    fmt::{self, Debug, Formatter},
    slice,
};

#[derive(Clone)]
pub struct ListIter<'data> {
    iter: slice::Iter<'data, usize>,
    data: &'data Data,
}

impl<'data> ListIter<'data> {
    fn new(iter: slice::Iter<'data, usize>, data: &'data Data) -> Self {
        Self { data, iter }
    }
}

impl<'data> Debug for ListIter<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ next: {:?} }}", self.clone().next())
    }
}

impl<'data> Iterator for ListIter<'data> {
    type Item = Node<'data>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|i| Node::new(self.data.get(*i), self.data))
    }
}

#[derive(Clone, Copy, Eq)]
pub struct ListNode<'data> {
    mark: Mark,
    cell: &'data ListCell,
    data: &'data Data,
}

impl<'data> ListNode<'data> {
    pub(super) fn new(mark: Mark, cell: &'data ListCell, data: &'data Data) -> Self {
        Self { mark, cell, data }
    }

    pub(super) fn debug(&self, f: &mut Formatter<'_>) -> fmt::Result {
        ListCell::debug((self.cell, self.data), f)
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn len(&self) -> usize {
        self.cell.data.len()
    }

    pub fn get(&self, index: usize) -> Result<Node<'data>, marked::InvalidIndexError> {
        match self.cell.data.get(index) {
            Some(i) => Ok(Node::new(self.data.get(*i), self.data)),
            None => Err(marked::WithMarkError::new(
                self.mark,
                InvalidIndexError::new(index, self.len()),
            )),
        }
    }

    pub fn iter(&self) -> ListIter<'data> {
        ListIter::new(self.cell.data.iter(), self.data)
    }
}

impl<'data> PartialEq for ListNode<'data> {
    fn eq(&self, other: &Self) -> bool {
        ListCell::equal((self.cell, self.data), (other.cell, other.data))
    }
}

impl<'data> Debug for ListNode<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ListNode {{ mark: {:?}, cell: ", self.mark)?;
        self.debug(f)?;
        write!(f, " }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{
        cell::{data_cell::DataCell, MarkedDataCell},
        node_type::NodeType,
    };

    fn test_data() -> Data {
        Data::new([
            MarkedDataCell::new(DataCell::String("hello".into()), Default::default()),
            MarkedDataCell::new(DataCell::Raw("hello".into()), Default::default()),
            MarkedDataCell::new(
                DataCell::List(ListCell::new(vec![0, 1])),
                Default::default(),
            ),
        ])
    }

    #[test]
    fn test_list_node() {
        let data = test_data();
        if let DataCell::List(cell) = &data.get(2).cell {
            let list = ListNode::new(Default::default(), cell, &data);

            let first = list.get(0).unwrap();
            assert_eq!(first.node_type(), NodeType::String);
            assert_eq!(first.string(), Ok("hello"));

            let second = list.get(1).unwrap();
            assert_eq!(second.node_type(), NodeType::Raw);
            assert_eq!(second.raw(), Ok("hello"));

            assert_eq!(list.len(), 2);

            let mut iter = list.iter();

            let first = iter.next().unwrap();
            assert_eq!(first.node_type(), NodeType::String);
            assert_eq!(first.string(), Ok("hello"));

            let second = iter.next().unwrap();
            assert_eq!(second.node_type(), NodeType::Raw);
            assert_eq!(second.raw(), Ok("hello"));

            assert!(iter.next().is_none());
        } else {
            panic!("The cell is not a list");
        }
    }
}
