use super::super::{
    super::{
        data::Data,
        error::{marked, InvalidIndexError},
        mark::Mark,
        node::node::ListNode,
    },
    analyse_anchors::AnalyseAnchors,
    view::View,
};
use std::{
    fmt::{self, Debug, Formatter},
    slice,
};

#[derive(Clone)]
pub struct ListIter<'data, A: AnalyseAnchors<'data>> {
    iter: slice::Iter<'data, usize>,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> ListIter<'data, A> {
    fn new(iter: slice::Iter<'data, usize>, data: &'data Data, anchor_analyser: A) -> Self {
        Self {
            data,
            iter,
            anchor_analyser,
        }
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for ListIter<'data, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ next: {:?} }}", self.clone().next())
    }
}

impl<'data, A: AnalyseAnchors<'data>> Iterator for ListIter<'data, A> {
    type Item = View<'data, A>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| {
            let node = self.data.get(*i);
            View::new(node, self.data, self.anchor_analyser.clone())
        })
    }
}

#[derive(Clone, Eq)]
pub struct ListView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data ListNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> ListView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data ListNode,
        data: &'data Data,
        anchor_analyser: A,
    ) -> Self {
        Self {
            mark,
            node,
            data,
            anchor_analyser,
        }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn len(&self) -> usize {
        self.node.data.len()
    }

    pub fn get(&self, index: usize) -> Result<View<'data, A>, marked::InvalidIndexError> {
        match self.node.data.get(index) {
            Some(i) => Ok({
                let node = self.data.get(*i);
                View::new(node, self.data, self.anchor_analyser.clone())
            }),
            None => Err({
                let error = InvalidIndexError::new(index, self.len());
                marked::WithMarkError::new(self.mark, error)
            }),
        }
    }

    pub fn iter(&self) -> ListIter<'data, A> {
        let anchor_analyser = self.anchor_analyser.clone();
        ListIter::new(self.node.data.iter(), self.data, anchor_analyser)
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for ListView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|(i, j)| i == j)
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for ListView<'data, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ListView {{ mark: {:?}, list: [", self.mark)?;
        for i in self.iter() {
            write!(f, "{:?}, ", i)?;
        }
        write!(f, "] }}")
    }
}

impl<'data, A: AnalyseAnchors<'data>> IntoIterator for ListView<'data, A> {
    type IntoIter = ListIter<'data, A>;
    type Item = View<'data, A>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter::new(self.node.data.iter(), self.data, self.anchor_analyser)
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
            let list = ListView::new(Default::default(), node, &data, ());

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
