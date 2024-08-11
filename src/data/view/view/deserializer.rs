use super::super::super::error::*;
use super::*;
use crate::de::parse::utils::to_value::*;
use serde::{
    de::{self, VariantAccess},
    Deserializer,
};

fn deserialize_number<'data, A, T>(
    deserializer: View<'data, A>,
    expected: &'static str,
) -> Result<T, marked::DeserializeError<CustomError>>
where
    A: AnalyseAnchors<'data>,
    T: ToNumber,
{
    let raw = deserializer.raw()?;
    to_number::<T>(raw.raw()).ok_or_else(|| {
        let invalid_value = InvalidValueError::new_expected(expected.into());
        marked::DeserializeError::new(raw.mark(), invalid_value.into())
    })
}

impl<'data, A: AnalyseAnchors<'data>> de::Deserializer<'data> for View<'data, A> {
    type Error = marked::DeserializeError<CustomError>;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        match self.to_match() {
            ToMatchView::Null(_) => visitor.visit_none(),
            ToMatchView::Raw(i) => visitor.visit_borrowed_bytes(i.raw().as_bytes()),
            ToMatchView::String(i) => visitor.visit_borrowed_str(i.string()),
            ToMatchView::List(i) => visitor.visit_seq(i.seq_access()),
            ToMatchView::Map(i) => visitor.visit_map(i.access()),
            ToMatchView::Tagged(i) => visitor.visit_enum(i),
            ToMatchView::Document(i) => i.view().deserialize_any(visitor),
            ToMatchView::Anchor(i) => i.view().deserialize_any(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let raw = self.raw()?;
        let boolean = to_bool(raw.raw()).ok_or_else(|| {
            let expected = "boolean value".into();
            let invalid_value = InvalidValueError::new_expected(expected);
            marked::DeserializeError::new(raw.mark(), invalid_value.into())
        })?;
        visitor.visit_bool(boolean)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^7 to 2^7 - 1";
        let number = deserialize_number::<_, i8>(self, expected)?;
        visitor.visit_i8(number)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^15 to 2^15 - 1";
        let number = deserialize_number::<_, i16>(self, expected)?;
        visitor.visit_i16(number)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^31 to 2^31 - 1";
        let number = deserialize_number::<_, i32>(self, expected)?;
        visitor.visit_i32(number)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^64 to 2^64 - 1";
        let number = deserialize_number::<_, i64>(self, expected)?;
        visitor.visit_i64(number)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^128 to 2^128 - 1";
        let number = deserialize_number::<_, i128>(self, expected)?;
        visitor.visit_i128(number)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^8 - 1";
        let number = deserialize_number::<_, u8>(self, expected)?;
        visitor.visit_u8(number)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^16 - 1";
        let number = deserialize_number::<_, u16>(self, expected)?;
        visitor.visit_u16(number)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^32 - 1";
        let number = deserialize_number::<_, u32>(self, expected)?;
        visitor.visit_u32(number)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^64 - 1";
        let number = deserialize_number::<_, u64>(self, expected)?;
        visitor.visit_u64(number)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^128 - 1";
        let number = deserialize_number::<_, u128>(self, expected)?;
        visitor.visit_u128(number)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let number = deserialize_number::<_, f32>(self, "number")?;
        visitor.visit_f32(number)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let number = deserialize_number::<_, f64>(self, "number")?;
        visitor.visit_f64(number)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let string = self.string()?;
        let mut chars = string.string().chars();
        match (chars.next(), chars.next()) {
            (Some(i), None) => visitor.visit_char(i),

            _ => {
                let expected = "one-character string".into();
                let invalid_value = InvalidValueError::new_expected(expected);
                let error = marked::DeserializeError::new(string.mark(), invalid_value.into());
                Err(error)
            }
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_borrowed_str(self.string()?.string())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_string(self.string()?.string().into())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_borrowed_bytes(self.raw()?.raw().as_bytes())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let list = self.list()?;
        let mut result = Vec::with_capacity(list.len());
        for i in list {
            let expected = "integer in the range from 0 to 2^8 - 1";
            let byte = deserialize_number::<_, u8>(i, expected).map_err(|i| {
                let expected = "byte sequence".into();
                let invalid_value = InvalidValueError::new(expected, Box::new(i));
                marked::DeserializeError::new(self.mark(), invalid_value.into())
            })?;
            result.push(byte);
        }
        visitor.visit_byte_buf(result)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let view = match self.tagged().map(TaggedView::split) {
            Ok(("Some", view)) => view,

            _ => match self.null() {
                Ok(_) => return visitor.visit_none(),
                Err(_) => self.clone(),
            },
        };

        visitor.visit_some(view).map_err(|error| {
            let expected = "optional value".into();
            let invalid_value = super::InvalidValueError::new(expected, Box::new(error));
            marked::DeserializeError::new(self.mark(), invalid_value.into())
        })
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        self.unit_variant()?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let (_, view) = self.tagged()?.verified(name)?;
        view.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let (_, view) = self.tagged()?.verified(name)?;
        visitor.visit_newtype_struct(view)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_seq(self.list()?.seq_access())
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let list = self.list()?;
        if list.len() == len {
            visitor.visit_seq(list.seq_access())
        } else {
            let invalid_length = InvalidLengthError::new(list.len());
            let error = marked::DeserializeError::new(list.mark(), invalid_length.into());
            Err(error)
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let (_, view) = self.tagged()?.verified(name)?;
        view.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let view = self.clear();
        match view.to_match() {
            ToMatchView::List(i) => visitor.visit_map(i.map_access()),
            ToMatchView::Map(i) => visitor.visit_map(i.access()),
            _ => {
                let expected = &[
                    NodeType::Map,
                    NodeType::List,
                    NodeType::Tagged,
                    NodeType::Anchor,
                    NodeType::Document,
                ];
                Err(self.make_invalid_type_error(expected).into())
            }
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let (_, view) = self.tagged()?.verified(name)?;
        let map = view.map()?;
        visitor.visit_map(map.access())
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let (_, view) = self.tagged()?.verified(name)?;
        let tagged = view.tagged()?;
        visitor.visit_enum(tagged)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_borrowed_str(self.raw()?.raw())
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_unit()
    }
}

impl<'data, A: AnalyseAnchors<'data>> de::VariantAccess<'data> for View<'data, A> {
    type Error = marked::DeserializeError<CustomError>;

    fn unit_variant(self) -> Result<(), Self::Error> {
        let list = self.list()?;
        if !list.is_empty() {
            let expected = "zero-length list".into();
            let invalid_value = InvalidValueError::new_expected(expected);
            let error = marked::DeserializeError::new(list.mark(), invalid_value.into());
            Err(error)
        } else {
            Ok(())
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'data>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let map = self.map()?;
        visitor.visit_map(map.access())
    }
}
