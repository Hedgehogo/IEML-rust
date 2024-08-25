use super::super::{buffer_anchors::BufferAnchors, Deserializer, Error, Result};
use crate::data::view::type_view::list_view::{ListIter, ListView};
use serde::de;

pub(crate) struct SeqAccess<'data, B: BufferAnchors<'data>> {
    iter: ListIter<'data, B>,
}

impl<'data, B: BufferAnchors<'data>> SeqAccess<'data, B> {
    pub(crate) fn new(view: ListView<'data, B>) -> Self {
        Self { iter: view.iter() }
    }
}

impl<'data, A: BufferAnchors<'data>> de::SeqAccess<'data> for SeqAccess<'data, A> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'data>,
    {
        match self.iter.next() {
            Some(i) => Ok(Some(seed.deserialize(Deserializer::new(i))?)),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}
