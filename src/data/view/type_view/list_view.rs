//! Type definition [`ListView`]

use super::super::{
    super::{
        data::Data,
        error::{
            invalid_length::Origin, marked, CustomError, InvalidLengthError, InvalidValueError,
        },
        mark::Mark,
        node::node::ListNode,
    },
    analyse_anchors::AnalyseAnchors,
    buffer_anchors::BufferAnchors,
    view::View,
};
use serde::de;
use std::{
    fmt::{self, Debug, Formatter},
    slice,
};

type Error = marked::DeserializeError<CustomError>;

/// Structure for reading List node data.
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

    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        self.mark
    }

    /// Asks if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.node.data.is_empty()
    }

    /// Gets the number of elements.
    pub fn len(&self) -> usize {
        self.node.data.len()
    }

    /// Gets the view on the element by the passed index.
    ///
    /// # Arguments
    /// * `index` Index of the requested item.
    pub fn get(&self, index: usize) -> Result<View<'data, A>, marked::InvalidLengthError> {
        match self.node.data.get(index) {
            Some(i) => {
                let node = self.data.get(*i);
                Ok(View::new(node, self.data, self.anchor_analyser.clone()))
            }
            None => {
                let error = InvalidLengthError::new(self.len(), Some(Origin::List), Some(index));
                Err(marked::MarkedError::new(self.mark, error))
            }
        }
    }

    /// Iterator by view per element.
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

impl<'data, A: BufferAnchors<'data>> ListView<'data, A> {
    pub(in super::super) fn access(self) -> impl de::SeqAccess<'data, Error = Error> {
        SeqAccess::new(self.iter())
    }

    pub(in super::super) fn map_access(self) -> impl de::MapAccess<'data, Error = Error> {
        MapAccess::new(self.mark, self.iter())
    }
}

struct SeqAccess<'data, A: BufferAnchors<'data>> {
    iter: ListIter<'data, A>,
}

impl<'data, A: BufferAnchors<'data>> SeqAccess<'data, A> {
    fn new(iter: ListIter<'data, A>) -> Self {
        Self { iter }
    }
}

impl<'data, A: BufferAnchors<'data>> de::SeqAccess<'data> for SeqAccess<'data, A> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'data>,
    {
        match self.iter.next() {
            Some(i) => Ok(Some(seed.deserialize(i)?)),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.iter.len())
    }
}

struct MapAccess<'data, A: BufferAnchors<'data>> {
    mark: Mark,
    last: Option<(usize, usize)>,
    iter: slice::Iter<'data, usize>,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: BufferAnchors<'data>> MapAccess<'data, A> {
    fn new(mark: Mark, iter: ListIter<'data, A>) -> Self {
        Self {
            mark,
            last: None,
            data: iter.data,
            iter: iter.iter,
            anchor_analyser: iter.anchor_analyser,
        }
    }

    fn next(&mut self) -> Result<(), marked::DeserializeError<CustomError>> {
        let result = self.iter.next().map(|i| {
            let node = self.data.get(*i);
            View::new(node, self.data, self.anchor_analyser.clone())
        });

        match result {
            Some(i) => {
                let list = i.list()?;
                let mut iter = list.node.data.iter();
                match (iter.next(), iter.next(), iter.next()) {
                    (Some(i), Some(j), None) => self.last = Some((*i, *j)),

                    _ => {
                        let length = list.len();
                        let origin = Some(Origin::List);
                        let invalid_length =
                            InvalidLengthError::new(length, origin, Some(2)).into();
                        let error = marked::DeserializeError::new(list.mark(), invalid_length);
                        return Err(error);
                    }
                }
            }

            None => self.last = None,
        }

        Ok(())
    }

    fn last_key(&self) -> Option<View<'data, A>> {
        self.last.map(|(i, _)| {
            let node = self.data.get(i);
            View::new(node, self.data, self.anchor_analyser.clone())
        })
    }

    fn last_value(&self) -> Option<View<'data, A>> {
        self.last.map(|(_, i)| {
            let node = self.data.get(i);
            View::new(node, self.data, self.anchor_analyser.clone())
        })
    }
}

impl<'data, A: BufferAnchors<'data>> de::MapAccess<'data> for MapAccess<'data, A> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'data>,
    {
        self.next()?;
        match self.last_key() {
            Some(i) => seed.deserialize(i).map(Some),

            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'data>,
    {
        match self.last_value() {
            Some(i) => seed.deserialize(i),

            None => {
                let invalid_value = InvalidValueError::new("value".into(), None);
                let error = Error::new(self.mark, invalid_value.into());
                Err(error)
            }
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
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
