//! Type definition [`RawView`]

use super::super::super::{
    error::{marked, CustomError, InvalidTypeError, InvalidValueError},
    mark::Mark,
    node::node::RawNode,
    node_type::NodeType,
};
use crate::data::error::expected::Expected;
use serde::de::{self, value::StrDeserializer};

/// Structure for reading Raw node data.
#[derive(Debug, Clone, Eq)]
pub struct RawView<'data> {
    mark: Mark,
    raw: &'data RawNode,
}

impl<'data> RawView<'data> {
    pub(in super::super) fn new(mark: Mark, raw: &'data RawNode) -> Self {
        Self { mark, raw }
    }

    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        self.mark
    }

    /// Gets the raw data as a string.
    pub fn raw(&self) -> &'data str {
        self.raw.as_str()
    }

    /// Checks if the content is equal to one of the expected tags.
    ///
    /// Returns the index of the matched tag.
    pub fn verify<E>(
        &self,
        expected: impl Into<Expected<&'static str>>,
    ) -> Result<usize, marked::InvalidValueError<E>> {
        let expected = Into::<Expected<&'static str>>::into(expected);

        for (i, &expected) in expected.iter().enumerate() {
            if expected == self.raw() {
                return Ok(i);
            }
        }

        let error = InvalidValueError::new(self.raw().into(), None);
        Err(marked::InvalidValueError::new(self.mark, error))
    }
}

impl<'data> PartialEq for RawView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.raw.as_str() == other.raw.as_str()
    }
}

impl<'data> de::EnumAccess<'data> for RawView<'data> {
    type Error = marked::DeserializeError<CustomError>;

    type Variant = RawView<'data>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'data>,
    {
        let variant_deserializer = StrDeserializer::<Self::Error>::new(self.raw());
        match seed.deserialize(variant_deserializer) {
            Ok(i) => Ok((i, self)),
            Err(i) => Err(marked::MarkedError::new(self.mark(), i.data)),
        }
    }
}

fn variant_error(view: RawView) -> marked::DeserializeError<CustomError> {
    let expected = &[NodeType::Tagged, NodeType::Anchor, NodeType::Document];
    let invalid_type = InvalidTypeError::new(NodeType::Raw, expected as &_);
    marked::DeserializeError::new(view.mark(), invalid_type.into())
}

impl<'data> de::VariantAccess<'data> for RawView<'data> {
    type Error = marked::DeserializeError<CustomError>;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'data>,
    {
        Err(variant_error(self))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        Err(variant_error(self))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        Err(variant_error(self))
    }
}
