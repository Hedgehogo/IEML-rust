//! Type definition [`MapView`]

use super::super::{
    super::{
        data::Data,
        error::{marked, MissingKeyError},
        mark::Mark,
        name::Name,
        node::map_node::MapNode,
    },
    analyse_anchors::AnalyseAnchors,
    view::View,
};
use std::{
    collections::hash_map,
    fmt::{self, Debug, Formatter}, borrow::Borrow,
};

#[derive(Clone)]
pub struct MapIter<'data, A: AnalyseAnchors<'data>> {
    iter: hash_map::Iter<'data, Name<Box<str>>, usize>,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> MapIter<'data, A> {
    fn new(
        iter: hash_map::Iter<'data, Name<Box<str>>, usize>,
        data: &'data Data,
        anchor_analyser: A,
    ) -> Self {
        Self {
            data,
            iter,
            anchor_analyser,
        }
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
        MapIter::new(self.node.data.iter(), self.data, anchor_analyser)
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
                .and_then(|i| (i == v).then(|| ()))
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
        MapIter::new(self.node.data.iter(), self.data, self.anchor_analyser)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{
        node::node::{MarkedNode, Node, TaggedNode},
        node_type::NodeType,
    };
    use std::{collections::HashMap, borrow::Borrow};

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
