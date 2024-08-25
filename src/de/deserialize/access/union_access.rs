use super::super::{buffer_anchors::BufferAnchors, Error, Result};
use super::struct_access::StructAccess;
use crate::data::{error::invalid_length::Origin, view::TaggedView};
use serde::de::{self, value::StrDeserializer, Deserializer};

pub(crate) struct UnionAccess<'data, B: BufferAnchors<'data>> {
    view: TaggedView<'data, B>,
}

impl<'data, B: BufferAnchors<'data>> UnionAccess<'data, B> {
    pub(crate) fn new(view: TaggedView<'data, B>) -> Self {
        Self { view }
    }

    fn deserializer(self) -> super::super::Deserializer<'data, B> {
        super::super::Deserializer::new(self.view.view())
    }
}

impl<'data, A: BufferAnchors<'data>> de::EnumAccess<'data> for UnionAccess<'data, A> {
    type Error = Error;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'data>,
    {
        let variant_deserializer = StrDeserializer::<Self::Error>::new(self.view.tag().as_str());
        match seed.deserialize(variant_deserializer) {
            Ok(i) => Ok((i, self)),
            Err(i) => Err(Error::new(self.view.mark(), i.data)),
        }
    }
}

impl<'data, A: BufferAnchors<'data>> de::VariantAccess<'data> for UnionAccess<'data, A> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        let list = self.view.view().list()?;
        if !list.is_empty() {
            let expected = Some(0);
            let origin = Some(Origin::List);
            let error = Error::new_invalid_length(list.mark(), list.len(), origin, expected);
            Err(error)
        } else {
            Ok(())
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'data>,
    {
        seed.deserialize(self.deserializer())
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        self.deserializer().deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let map = self.view.view().map()?;
        visitor.visit_map(StructAccess::new(map))
    }
}
