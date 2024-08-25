use super::super::{Error, Result};
use crate::data::{
    error::{DeserializeError, InvalidTypeError, UnknownRawError},
    node_type::NodeType,
    view::RawView,
};
use serde::de::{self, value::StrDeserializer};

pub(crate) struct EnumAccess<'data> {
    view: RawView<'data>,
}

impl<'data> EnumAccess<'data> {
    pub(crate) fn new(view: RawView<'data>) -> Self {
        Self { view }
    }

    fn error(&self) -> Error {
        let expected = &[NodeType::Tagged, NodeType::Anchor, NodeType::Document];
        let invalid_type = InvalidTypeError::new(NodeType::Raw, expected as &_);
        Error::new(self.view.mark(), invalid_type.into())
    }
}

impl<'data> de::EnumAccess<'data> for EnumAccess<'data> {
    type Error = Error;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'data>,
    {
        let variant_deserializer = StrDeserializer::<Self::Error>::new(self.view.raw());
        match seed.deserialize(variant_deserializer) {
            Ok(i) => Ok((i, self)),

            Err(i) => {
                let error = match i.data {
                    DeserializeError::UnknownTag(i) => UnknownRawError::from(i).into(),
                    i => i,
                };

                Err(Error::new(self.view.mark(), error))
            }
        }
    }
}

impl<'data> de::VariantAccess<'data> for EnumAccess<'data> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'data>,
    {
        Err(self.error())
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        Err(self.error())
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        Err(self.error())
    }
}
