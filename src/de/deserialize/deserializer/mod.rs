//! Type definition [`Deserializer`]

use super::super::parse::utils::to_value::{to_bool, to_number, ToNumber};
use super::{
    access::{
        anchor_access::AnchorAccess, enum_access::EnumAccess, map_access::MapAccess,
        seq_access::SeqAccess, struct_access::StructAccess, union_access::UnionAccess,
    },
    buffer_anchors::BufferAnchors,
};
use crate::data::{
    error::{invalid_length::Origin, *},
    node_type::NodeType,
    view::{ToMatchView, View},
};
use serde::de::{self, value::UnitDeserializer, VariantAccess};

pub type Error = marked::DeserializeError<CustomError>;
pub type Result<T> = std::result::Result<T, Error>;

/// Structure for deserialisation node data.
#[derive(Clone, Copy)]
pub struct Deserializer<'data, B: BufferAnchors<'data>> {
    view: View<'data, B>,
}

impl<'data, B: BufferAnchors<'data>> Deserializer<'data, B> {
    pub fn new(view: View<'data, B>) -> Self {
        Self { view }
    }

    fn deserialize_number<T: ToNumber>(self, expected: String) -> Result<T> {
        let raw = self.view.raw()?;
        to_number::<T>(raw.raw()).ok_or_else(|| {
            let invalid_value = InvalidValueError::new(expected, None);
            marked::DeserializeError::new(raw.mark(), invalid_value.into())
        })
    }

    fn deserialize_type<F, T>(self, name: &'static str, f: F) -> Result<T>
    where
        F: FnOnce(Self) -> Result<T>,
    {
        let mark = self.view.mark();
        f(self).map_err(|i| {
            let expected = format!("value of type {:?}", name);
            let invalid_value = InvalidValueError::new(expected, Some(Box::new(i)));
            marked::DeserializeError::new(mark, invalid_value.into())
        })
    }
}

impl<'data, B: BufferAnchors<'data>> de::Deserializer<'data> for Deserializer<'data, B> {
    type Error = marked::DeserializeError<CustomError>;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        match self.view.to_match() {
            ToMatchView::Null(_) => visitor.visit_none(),
            ToMatchView::Raw(i) => visitor.visit_borrowed_bytes(i.raw().as_bytes()),
            ToMatchView::String(i) => visitor.visit_borrowed_str(i.string()),
            ToMatchView::List(i) => visitor.visit_seq(SeqAccess::new(i)),
            ToMatchView::Map(i) => visitor.visit_map(StructAccess::new(i)),
            ToMatchView::Tagged(i) => visitor.visit_enum(UnionAccess::new(i)),
            ToMatchView::Document(i) => Deserializer::new(i.view()).deserialize_any(visitor),
            ToMatchView::Anchor(i) => Deserializer::new(i.view()).deserialize_any(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let raw = self.view.raw()?;
        let boolean = to_bool(raw.raw()).ok_or_else(|| {
            let expected = "boolean value".into();
            let invalid_value = InvalidValueError::new(expected, None);
            marked::DeserializeError::new(raw.mark(), invalid_value.into())
        })?;
        visitor.visit_bool(boolean)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^7 to 2^7 - 1".into();
        let number = self.deserialize_number::<i8>(expected)?;
        visitor.visit_i8(number)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^15 to 2^15 - 1".into();
        let number = self.deserialize_number::<i16>(expected)?;
        visitor.visit_i16(number)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^31 to 2^31 - 1".into();
        let number = self.deserialize_number::<i32>(expected)?;
        visitor.visit_i32(number)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^64 to 2^64 - 1".into();
        let number = self.deserialize_number::<i64>(expected)?;
        visitor.visit_i64(number)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^128 to 2^128 - 1".into();
        let number = self.deserialize_number::<i128>(expected)?;
        visitor.visit_i128(number)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^8 - 1".into();
        let number = self.deserialize_number::<u8>(expected)?;
        visitor.visit_u8(number)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^16 - 1".into();
        let number = self.deserialize_number::<u16>(expected)?;
        visitor.visit_u16(number)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^32 - 1".into();
        let number = self.deserialize_number::<u32>(expected)?;
        visitor.visit_u32(number)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^64 - 1".into();
        let number = self.deserialize_number::<u64>(expected)?;
        visitor.visit_u64(number)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^128 - 1".into();
        let number = self.deserialize_number::<u128>(expected)?;
        visitor.visit_u128(number)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let number = self.deserialize_number::<f32>("number".into())?;
        visitor.visit_f32(number)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let number = self.deserialize_number::<f64>("number".into())?;
        visitor.visit_f64(number)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let string = self.view.string()?;
        let mut chars = string.string().chars();
        match (chars.next(), chars.next()) {
            (Some(i), None) => visitor.visit_char(i),

            _ => {
                let expected = "one-character string".into();
                let invalid_value = InvalidValueError::new(expected, None);
                let error = marked::DeserializeError::new(string.mark(), invalid_value.into());
                Err(error)
            }
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_borrowed_str(self.view.string()?.string())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_string(self.view.string()?.string().into())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_borrowed_bytes(self.view.raw()?.raw().as_bytes())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let list = self.view.list()?;
        let mut result = Vec::with_capacity(list.len());
        for i in list {
            let expected = "integer in the range from 0 to 2^8 - 1".into();
            let byte = Deserializer::new(i)
                .deserialize_number::<u8>(expected)
                .map_err(|i| {
                    let expected = "byte sequence".into();
                    let invalid_value = InvalidValueError::new(expected, Some(Box::new(i)));
                    marked::DeserializeError::new(self.view.mark(), invalid_value.into())
                })?;
            result.push(byte);
        }
        visitor.visit_byte_buf(result)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let deserialize = || {
            let view = self.view.clear_tag();

            let expected = &["Some", "None"] as &[_];
            match view.to_match() {
                ToMatchView::Tagged(i) => match i.verify(expected)? {
                    0 => return visitor.visit_some(Deserializer::new(i.view())),
                    _ => {
                        let access = UnionAccess::new(i);
                        return access.unit_variant().and_then(|_| visitor.visit_none());
                    }
                },

                ToMatchView::Raw(i) => match i.verify(expected)? {
                    0 => {
                        let unit_deserializer = UnitDeserializer::<Error>::new();
                        return visitor.visit_some(unit_deserializer).map_err(|_| {
                            use NodeType::*;
                            let expected = &[Tagged, Anchor, Document] as &[_];
                            let invalid_type = InvalidTypeError::new(Raw, expected);
                            marked::DeserializeError::new(view.mark(), invalid_type.into())
                        });
                    }

                    _ => return visitor.visit_none(),
                },

                ToMatchView::Null(_) => return visitor.visit_none(),

                _ => {}
            };

            visitor.visit_some(self.clone())
        };

        deserialize().map_err(|error| {
            let expected = "optional value".into();
            let invalid_value = InvalidValueError::new(expected, Some(Box::new(error)));
            marked::DeserializeError::new(self.view.mark(), invalid_value.into())
        })
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let list = self.view.list()?;
        if list.is_empty() {
            return visitor.visit_unit();
        }

        let invalid_length = InvalidLengthError::new(list.len(), Some(Origin::List), Some(0));
        let error = marked::DeserializeError::new(list.mark(), invalid_length.into());
        Err(error)
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        self.deserialize_type(name, |i| i.deserialize_unit(visitor))
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        match name.chars().next() {
            Some('@') => match self.view.anchor() {
                Ok(i) => {
                    let access = AnchorAccess::new(i.mark(), i.view(), Some(i.name().as_str()));
                    visitor.visit_some(access)
                }

                Err(_) => visitor.visit_some(AnchorAccess::new(self.view.mark(), self.view, None)),
            },

            _ => self.deserialize_type(name, |i| visitor.visit_newtype_struct(i)),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_seq(SeqAccess::new(self.view.list()?))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let list = self.view.list()?;
        if list.len() == len {
            visitor.visit_seq(SeqAccess::new(list))
        } else {
            let invalid_length = InvalidLengthError::new(list.len(), Some(Origin::List), Some(len));
            let error = marked::DeserializeError::new(list.mark(), invalid_length.into());
            Err(error)
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        self.deserialize_type(name, |i| i.deserialize_tuple(len, visitor))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        let view = self.view.clear();
        match view.to_match() {
            ToMatchView::List(i) => visitor.visit_map(MapAccess::new(i)),

            ToMatchView::Map(i) => visitor.visit_map(StructAccess::new(i)),

            _ => {
                let expected = &[
                    NodeType::Map,
                    NodeType::List,
                    NodeType::Tagged,
                    NodeType::Anchor,
                    NodeType::Document,
                ] as &[_];
                let error = Error::new_invalid_type(view.mark(), view.node_type(), expected);
                Err(error)
            }
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        self.deserialize_type(name, |i| {
            visitor.visit_map(StructAccess::new(i.view.map()?))
        })
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        self.deserialize_type(name, |i| match i.view.tagged() {
            Ok(i) => visitor.visit_enum(UnionAccess::new(i)),
            Err(_) => visitor.visit_enum(EnumAccess::new(i.view.raw()?)),
        })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_borrowed_str(self.view.raw()?.raw())
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_unit()
    }
}

#[cfg(test)]
mod tests;
