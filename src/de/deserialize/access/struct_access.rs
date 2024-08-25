use super::super::{buffer_anchors::BufferAnchors, Deserializer, Error, Result};
use crate::data::{
    error::DeserializeError,
    mark::Mark,
    name::Name,
    node_type::NodeType,
    view::{
        type_view::map_view::{MapIter, MapView},
        View,
    },
};
use serde::de::{self, value::StrDeserializer};

pub(crate) struct StructAccess<'data, B: BufferAnchors<'data>> {
    mark: Mark,
    last: Option<(&'data Name<Box<str>>, View<'data, B>)>,
    iter: MapIter<'data, B>,
}

impl<'data, B: BufferAnchors<'data>> StructAccess<'data, B> {
    pub(crate) fn new(view: MapView<'data, B>) -> Self {
        Self {
            mark: view.mark(),
            last: None,
            iter: view.iter(),
        }
    }

    fn next(&mut self) {
        self.last = self.iter.next().map(|(key, i)| (key, i));
    }

    fn last_key(&self) -> Option<&'data Name<Box<str>>> {
        self.last.as_ref().map(|(key, _)| *key)
    }

    fn last_value(&self) -> Option<View<'data, B>> {
        self.last.as_ref().map(|(_, i)| i.clone())
    }
}

impl<'data, B: BufferAnchors<'data>> de::MapAccess<'data> for StructAccess<'data, B> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'data>,
    {
        self.next();
        match self.last_key() {
            Some(i) => match seed.deserialize(StrDeserializer::<Self::Error>::new(i.as_ref())) {
                Ok(i) => Ok(Some(i)),

                Err(i) => match i.data {
                    DeserializeError::InvalidType(_) => {
                        let expected = &[NodeType::List] as &[_];
                        let error = Error::new_invalid_type(self.mark, NodeType::Map, expected);
                        Err(error)
                    }

                    _ => Err(Error::new(self.mark, i.data)),
                },
            },

            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'data>,
    {
        match self.last_value() {
            Some(i) => return seed.deserialize(Deserializer::new(i)),

            None => {
                let error = Error::new_invalid_value(self.mark, "value".into(), None);
                Err(error)
            }
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}
