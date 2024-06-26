use super::super::{
    super::{
        data::Data,
        error::{marked, InvalidIndexError},
        mark::Mark,
        node::node::ListNode,
    },
    view::View,
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
    type Item = View<'data>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|i| View::new(self.data.get(*i), self.data))
    }
}

#[derive(Clone, Copy, Eq)]
pub struct ListView<'data> {
    mark: Mark,
    node: &'data ListNode,
    data: &'data Data,
}

impl<'data> ListView<'data> {
    pub(in super::super) fn new(mark: Mark, node: &'data ListNode, data: &'data Data) -> Self {
        Self { mark, node, data }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn len(&self) -> usize {
        self.node.data.len()
    }

    pub fn get(&self, index: usize) -> Result<View<'data>, marked::InvalidIndexError> {
        match self.node.data.get(index) {
            Some(i) => Ok(View::new(self.data.get(*i), self.data)),
            None => Err(marked::WithMarkError::new(
                self.mark,
                InvalidIndexError::new(index, self.len()),
            )),
        }
    }

    pub fn iter(&self) -> ListIter<'data> {
        ListIter::new(self.node.data.iter(), self.data)
    }
}

impl<'data> PartialEq for ListView<'data> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|(i, j)| i == j)
    }
}

impl<'data> Debug for ListView<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ListView {{ mark: {:?}, list: [", self.mark)?;
        for i in self.iter() {
            write!(f, "{:?}, ", i)?;
        }
        write!(f, "] }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{
        node::node::{MarkedNode, Node},
        node_type::NodeType,
    };

    fn test_data() -> Data {
        Data::new([
            MarkedNode::new(Node::String("hello".into()), Default::default()),
            MarkedNode::new(Node::Raw("hello".into()), Default::default()),
            MarkedNode::new(Node::List(ListNode::new(vec![0, 1])), Default::default()),
        ])
    }

    #[test]
    fn test_list_view() {
        let data = test_data();
        if let Node::List(node) = &data.get(2).node {
            let list = ListView::new(Default::default(), node, &data);

            let first = list.get(0).unwrap();
            assert_eq!(first.node_type(), NodeType::String);
            assert_eq!(first.string().unwrap().string(), "hello");

            let second = list.get(1).unwrap();
            assert_eq!(second.node_type(), NodeType::Raw);
            assert_eq!(second.raw().unwrap().raw(), "hello");

            assert_eq!(list.len(), 2);

            let mut iter = list.iter();

            let first = iter.next().unwrap();
            assert_eq!(first.node_type(), NodeType::String);
            assert_eq!(first.string().unwrap().string(), "hello");

            let second = iter.next().unwrap();
            assert_eq!(second.node_type(), NodeType::Raw);
            assert_eq!(second.raw().unwrap().raw(), "hello");

            assert!(iter.next().is_none());
        } else {
            panic!("The node is not a list");
        }
    }
}
