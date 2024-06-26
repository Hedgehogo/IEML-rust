use super::{
    super::super::{
        cell::{map_cell::MapCell, Data},
        error::{marked, InvalidKeyError},
        mark::Mark,
    },
    View,
};
use std::{
    collections::hash_map,
    fmt::{self, Debug, Formatter},
};

#[derive(Clone)]
pub struct MapIter<'data> {
    iter: hash_map::Iter<'data, String, usize>,
    data: &'data Data,
}

impl<'data> MapIter<'data> {
    fn new(iter: hash_map::Iter<'data, String, usize>, data: &'data Data) -> Self {
        Self { data, iter }
    }
}

impl<'data> Debug for MapIter<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ next: {:?} }}", self.clone().next())
    }
}

impl<'data> Iterator for MapIter<'data> {
    type Item = (&'data String, View<'data>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(key, i)| (key, View::new(self.data.get(*i), self.data)))
    }
}

#[derive(Clone, Copy, Eq)]
pub struct MapView<'data> {
    mark: Mark,
    cell: &'data MapCell,
    data: &'data Data,
}

impl<'data> MapView<'data> {
    pub(in super::super) fn new(mark: Mark, cell: &'data MapCell, data: &'data Data) -> Self {
        Self { mark, cell, data }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn len(&self) -> usize {
        self.cell.data.len()
    }

    pub fn contains_key(&self, key: &String) -> bool {
        self.cell.data.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Result<View<'data>, marked::InvalidKeyError> {
        match self.cell.data.get(key) {
            Some(i) => Ok(View::new(self.data.get(*i), self.data)),
            None => Err(marked::WithMarkError::new(
                self.mark,
                InvalidKeyError::new(key.into()),
            )),
        }
    }

    pub fn iter(&self) -> MapIter<'data> {
        MapIter::new(self.cell.data.iter(), self.data)
    }
}

impl<'data> PartialEq for MapView<'data> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().all(|(k, v)| {
            other
                .get(k)
                .ok()
                .and_then(|i| (i == v).then(|| ()))
                .is_some()
        })
    }
}

impl<'data> Debug for MapView<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MapView {{ mark: {:?}, map: [", self.mark)?;
        for (k, v) in self.iter() {
            write!(f, "{:?}: {:?}, ", k, v)?;
        }
        write!(f, "] }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{
        cell::{
            data_cell::{DataCell, TaggedCell},
            MarkedDataCell,
        },
        node_type::NodeType,
    };
    use std::collections::HashMap;

    fn test_data() -> Data {
        Data::new([
            MarkedDataCell::new(DataCell::Null, Default::default()),
            MarkedDataCell::new(DataCell::Null, Default::default()),
            MarkedDataCell::new(
                DataCell::Tagged(TaggedCell::new("tag".into(), 0)),
                Default::default(),
            ),
            MarkedDataCell::new(
                DataCell::Map(MapCell::new(HashMap::from([
                    ("first".into(), 1),
                    ("second".into(), 2),
                ]))),
                Default::default(),
            ),
        ])
    }

    #[test]
    fn test_map_view() {
        let data = test_data();
        if let DataCell::Map(cell) = &data.get(3).cell {
            let list = MapView::new(Default::default(), cell, &data);

            let first = list.get("first").unwrap();
            assert_eq!(first.node_type(), NodeType::Null);

            let second = list.get("second").unwrap();
            assert_eq!(second.node_type(), NodeType::Tagged);
            assert_eq!(second.tagged().map(|i| i.tag()), Ok("tag"));

            assert_eq!(list.len(), 2);

            for (key, i) in list.iter() {
                assert!(list.contains_key(key));
                assert_eq!(list.get(key).unwrap(), i);
            }
        } else {
            panic!("The cell is not a map");
        }
    }
}
