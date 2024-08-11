//! Type definition [`MapView`]

use crate::data::{error::InvalidTypeError, node_type::NodeType};

use super::super::{
    super::{
        data::Data,
        error::{marked, CustomError, DeserializeError, InvalidValueError, MissingKeyError},
        mark::Mark,
        name::Name,
        node::map_node::MapNode,
    },
    analyse_anchors::AnalyseAnchors,
    view::View,
};
use serde::de::{self, value::StrDeserializer};
use std::{
    borrow::Borrow,
    collections::hash_map,
    fmt::{self, Debug, Formatter},
};

/// Structure for reading Map node data.
#[derive(Clone, Eq)]
pub struct MapView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data MapNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> MapView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data MapNode,
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

    /// Asks if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.node.data.is_empty()
    }

    /// Gets the number of elements.
    pub fn len(&self) -> usize {
        self.node.data.len()
    }

    /// Asks if a certain key is contained in the map.
    pub fn contains_key(&self, key: &str) -> bool {
        self.node.data.contains_key(key)
    }

    /// Gets the view on the element by the passed key.
    ///
    /// # Arguments
    /// * `key` Key of the requested item.
    pub fn get(&self, key: &str) -> Result<View<'data, A>, marked::MissingKeyError> {
        match self.node.data.get(key) {
            Some(i) => Ok({
                let node = self.data.get(*i);
                View::new(node, self.data, self.anchor_analyser.clone())
            }),
            None => Err({
                let error = MissingKeyError::new(key.into());
                marked::MarkedError::new(self.mark, error)
            }),
        }
    }

    /// Iterate over tuples of keys and view to the corresponding element.
    pub fn iter(&self) -> MapIter<'data, A> {
        let anchor_analyser = self.anchor_analyser.clone();
        MapIter::new(self.mark, self.node.data.iter(), self.data, anchor_analyser)
    }

    pub(crate) fn access(self) -> MapAccess<'data, A> {
        MapAccess::new(self.mark, self.iter())
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for MapView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().all(|(k, v)| {
            other
                .get(k.borrow())
                .ok()
                .and_then(|i| (i == v).then_some(()))
                .is_some()
        })
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for MapView<'data, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MapView {{ mark: {:?}, map: [", self.mark)?;
        for (k, v) in self.iter() {
            write!(f, "{:?}: {:?}, ", k, v)?;
        }
        write!(f, "] }}")
    }
}

impl<'data, A: AnalyseAnchors<'data>> IntoIterator for MapView<'data, A> {
    type IntoIter = MapIter<'data, A>;
    type Item = (&'data Name<Box<str>>, View<'data, A>);

    fn into_iter(self) -> Self::IntoIter {
        MapIter::new(
            self.mark,
            self.node.data.iter(),
            self.data,
            self.anchor_analyser,
        )
    }
}

#[derive(Clone)]
pub struct MapIter<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    iter: hash_map::Iter<'data, Name<Box<str>>, usize>,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> MapIter<'data, A> {
    fn new(
        mark: Mark,
        iter: hash_map::Iter<'data, Name<Box<str>>, usize>,
        data: &'data Data,
        anchor_analyser: A,
    ) -> Self {
        Self {
            mark,
            data,
            iter,
            anchor_analyser,
        }
    }

    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        self.mark
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for MapIter<'data, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ next: {:?} }}", self.clone().next())
    }
}

impl<'data, A: AnalyseAnchors<'data>> Iterator for MapIter<'data, A> {
    type Item = (&'data Name<Box<str>>, View<'data, A>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, i)| {
            let node = self.data.get(*i);
            let view = View::new(node, self.data, self.anchor_analyser.clone());
            (key, view)
        })
    }
}

pub struct MapAccess<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    last: Option<(&'data Name<Box<str>>, usize)>,
    iter: hash_map::Iter<'data, Name<Box<str>>, usize>,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> MapAccess<'data, A> {
    fn new(mark: Mark, iter: MapIter<'data, A>) -> Self {
        Self {
            mark: mark,
            last: None,
            data: iter.data,
            iter: iter.iter,
            anchor_analyser: iter.anchor_analyser,
        }
    }

    fn next(&mut self) {
        self.last = self.iter.next().map(|(key, i)| (key, *i));
    }

    fn last_key(&self) -> Option<&'data Name<Box<str>>> {
        self.last.map(|(key, _)| key)
    }

    fn last_value(&self) -> Option<View<'data, A>> {
        self.last.map(|(_, i)| {
            let node = self.data.get(i);
            View::new(node, self.data, self.anchor_analyser.clone())
        })
    }
}

impl<'data, A: AnalyseAnchors<'data>> de::MapAccess<'data> for MapAccess<'data, A> {
    type Error = marked::DeserializeError<CustomError>;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'data>,
    {
        self.next();
        match self.last_key() {
            Some(i) => match seed.deserialize(StrDeserializer::<Self::Error>::new(i.as_ref())) {
                Ok(i) => Ok(Some(i)),
                Err(i) => match i.data {
                    DeserializeError::InvalidType(_) => {
                        let invalid_type = InvalidTypeError::new(NodeType::Map, &[NodeType::List]);
                        let error = marked::DeserializeError::new(self.mark, invalid_type.into());
                        Err(error)
                    }
                    _ => {
                        let invalid_value = InvalidValueError::new("map".into(), Box::new(i));
                        let error = marked::DeserializeError::new(self.mark, invalid_value.into());
                        Err(error)
                    }
                },
            },
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'data>,
    {
        match self.last_value() {
            Some(i) => match seed.deserialize(i) {
                Ok(i) => Ok(i),
                Err(i) => {
                    let invalid_value = InvalidValueError::new("map".into(), Box::new(i));
                    let error = marked::DeserializeError::new(self.mark, invalid_value.into());
                    Err(error)
                }
            },
            None => {
                let invalid_value = InvalidValueError::new_expected("value".into());
                let error = marked::DeserializeError::new(self.mark, invalid_value.into());
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
        node::node::{MarkedNode, Node, TaggedNode},
        node_type::NodeType,
    };
    use std::{borrow::Borrow, collections::HashMap};

    fn test_data() -> Data {
        let name = |i: &str| Name::new(i.into()).unwrap();
        Data::new([
            MarkedNode::new(Node::Null, Default::default()),
            MarkedNode::new(Node::Null, Default::default()),
            MarkedNode::new(
                Node::Tagged(TaggedNode::new(name("tag"), 0)),
                Default::default(),
            ),
            MarkedNode::new(
                Node::Map(MapNode::new(HashMap::from([
                    (name("first"), 1),
                    (name("second"), 2),
                ]))),
                Default::default(),
            ),
        ])
    }

    #[test]
    fn test_map_view() {
        let data = test_data();
        if let Node::Map(node) = &data.get(3).node {
            let list = MapView::new(Default::default(), node, &data, ());

            let first = list.get("first").unwrap();
            assert_eq!(first.node_type(), NodeType::Null);

            let second = list.get("second").unwrap();
            assert_eq!(second.node_type(), NodeType::Tagged);
            assert_eq!(second.tagged().unwrap().tag().as_str(), "tag");

            assert_eq!(list.len(), 2);

            for (key, i) in list.iter() {
                assert!(list.contains_key(key.borrow()));
                assert_eq!(list.get(key.borrow()).unwrap(), i);
            }
        } else {
            panic!("The node is not a map");
        }
    }
}
