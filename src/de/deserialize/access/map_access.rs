use super::super::{buffer_anchors::BufferAnchors, Deserializer, Error, Result};
use crate::data::{
    error::invalid_length::Origin,
    mark::Mark,
    view::{
        type_view::list_view::{ListIter, ListView},
        View,
    },
};
use serde::de;

pub(crate) struct MapAccess<'data, B: BufferAnchors<'data>> {
    mark: Mark,
    last: Option<(View<'data, B>, View<'data, B>)>,
    iter: ListIter<'data, B>,
}

impl<'data, B: BufferAnchors<'data>> MapAccess<'data, B> {
    pub(crate) fn new(view: ListView<'data, B>) -> Self {
        Self {
            mark: view.mark(),
            last: None,
            iter: view.iter(),
        }
    }

    fn next(&mut self) -> Result<()> {
        let result = self.iter.next();
        match result {
            Some(i) => {
                let list = i.list()?;
                let mut iter = list.iter();
                match (iter.next(), iter.next(), iter.next()) {
                    (Some(i), Some(j), None) => self.last = Some((i, j)),

                    _ => {
                        let mark = list.mark();
                        let length = list.len();
                        let origin = Some(Origin::List);
                        let expected = Some(2);
                        let error = Error::new_invalid_length(mark, length, origin, expected);
                        return Err(error);
                    }
                }
            }

            None => self.last = None,
        }

        Ok(())
    }

    fn last_key(&self) -> Option<View<'data, B>> {
        self.last.as_ref().map(|(i, _)| i.clone())
    }

    fn last_value(&self) -> Option<View<'data, B>> {
        self.last.as_ref().map(|(_, i)| i.clone())
    }
}

impl<'data, A: BufferAnchors<'data>> de::MapAccess<'data> for MapAccess<'data, A> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'data>,
    {
        self.next()?;
        match self.last_key() {
            Some(i) => seed.deserialize(Deserializer::new(i)).map(Some),

            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'data>,
    {
        match self.last_value() {
            Some(i) => seed.deserialize(Deserializer::new(i)),

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
