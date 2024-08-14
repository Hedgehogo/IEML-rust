use super::super::super::error::*;
use super::*;
use crate::de::parse::utils::to_value::*;
use serde::de::{self, value::UnitDeserializer, VariantAccess};

fn deserialize_number<'data, A, T>(
    deserializer: View<'data, A>,
    expected: String,
) -> Result<T, marked::DeserializeError<CustomError>>
where
    A: AnalyseAnchors<'data>,
    T: ToNumber,
{
    let raw = deserializer.raw()?;
    to_number::<T>(raw.raw()).ok_or_else(|| {
        let invalid_value = InvalidValueError::new(expected, None);
        marked::DeserializeError::new(raw.mark(), invalid_value.into())
    })
}

fn deserialize_type<'data, A, F, T>(
    deserializer: View<'data, A>,
    name: &'static str,
    f: F,
) -> Result<T, marked::DeserializeError<CustomError>>
where
    A: AnalyseAnchors<'data>,
    F: FnOnce(View<'data, A>) -> Result<T, marked::DeserializeError<CustomError>>,
{
    let mark = deserializer.mark();
    f(deserializer).map_err(|i| {
        let expected = format!("value of type {:?}", name);
        let invalid_value = super::InvalidValueError::new(expected, Some(Box::new(i)));
        marked::DeserializeError::new(mark, invalid_value.into())
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
            ToMatchView::List(i) => visitor.visit_seq(i.access()),
            ToMatchView::Map(i) => visitor.visit_map(i.access()),
            ToMatchView::Tagged(i) => visitor.visit_enum(i.access()),
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
            let invalid_value = InvalidValueError::new(expected, None);
            marked::DeserializeError::new(raw.mark(), invalid_value.into())
        })?;
        visitor.visit_bool(boolean)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^7 to 2^7 - 1".into();
        let number = deserialize_number::<_, i8>(self, expected)?;
        visitor.visit_i8(number)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^15 to 2^15 - 1".into();
        let number = deserialize_number::<_, i16>(self, expected)?;
        visitor.visit_i16(number)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^31 to 2^31 - 1".into();
        let number = deserialize_number::<_, i32>(self, expected)?;
        visitor.visit_i32(number)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^64 to 2^64 - 1".into();
        let number = deserialize_number::<_, i64>(self, expected)?;
        visitor.visit_i64(number)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from -2^128 to 2^128 - 1".into();
        let number = deserialize_number::<_, i128>(self, expected)?;
        visitor.visit_i128(number)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^8 - 1".into();
        let number = deserialize_number::<_, u8>(self, expected)?;
        visitor.visit_u8(number)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^16 - 1".into();
        let number = deserialize_number::<_, u16>(self, expected)?;
        visitor.visit_u16(number)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^32 - 1".into();
        let number = deserialize_number::<_, u32>(self, expected)?;
        visitor.visit_u32(number)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^64 - 1".into();
        let number = deserialize_number::<_, u64>(self, expected)?;
        visitor.visit_u64(number)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let expected = "integer in the range from 0 to 2^128 - 1".into();
        let number = deserialize_number::<_, u128>(self, expected)?;
        visitor.visit_u128(number)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let number = deserialize_number::<_, f32>(self, "number".into())?;
        visitor.visit_f32(number)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let number = deserialize_number::<_, f64>(self, "number".into())?;
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
                let invalid_value = InvalidValueError::new(expected, None);
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
            let expected = "integer in the range from 0 to 2^8 - 1".into();
            let byte = deserialize_number::<_, u8>(i, expected).map_err(|i| {
                let expected = "byte sequence".into();
                let invalid_value = InvalidValueError::new(expected, Some(Box::new(i)));
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
        let deserialize = || {
            let view = self.clear_tag();

            let expected = &["Some", "None"] as &[_];
            match view.to_match() {
                ToMatchView::Tagged(i) => match i.verify(expected)? {
                    0 => return visitor.visit_some(i.view()),
                    _ => return i.access().unit_variant().and_then(|_| visitor.visit_none()),
                },

                ToMatchView::Raw(i) => match i.verify(expected)? {
                    0 => {
                        let unit_deserializer = UnitDeserializer::<Self::Error>::new();
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
            marked::DeserializeError::new(self.mark(), invalid_value.into())
        })
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let list = self.list()?;
        if list.is_empty() {
            return visitor.visit_unit();
        }

        let invalid_length = InvalidLengthError::new(list.len());
        let error = marked::DeserializeError::new(list.mark(), invalid_length.into());
        Err(error)
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let tagged = self.tagged()?;
        tagged.verify(name)?;
        tagged.view().deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        deserialize_type(self, name, |i| visitor.visit_newtype_struct(i))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        visitor.visit_seq(self.list()?.access())
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let list = self.list()?;
        if list.len() == len {
            visitor.visit_seq(list.access())
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
        deserialize_type(self, name, |i| i.deserialize_tuple(len, visitor))
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
        deserialize_type(self, name, |i| visitor.visit_map(i.map()?.access()))
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
        deserialize_type(self, name, |i| match i.tagged() {
            Ok(i) => visitor.visit_enum(i.access()),
            Err(_) => visitor.visit_enum(i.raw()?.access()),
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::collections::HashMap;

    #[test]
    fn test_number() {
        let data = crate::from_source("10").unwrap();
        let result = i32::deserialize(data.view());
        assert_eq!(result, Ok(10));

        let data = crate::from_source("hello").unwrap();
        let result = i32::deserialize(data.view());
        assert_eq!(
            result,
            Err(marked::DeserializeError::new_invalid_value(
                Mark::new(0, 0),
                "integer in the range from -2^31 to 2^31 - 1".into(),
                None
            ))
        );
    }

    #[test]
    fn test_char() {
        let data = crate::from_source("> h").unwrap();
        let result = char::deserialize(data.view());
        assert_eq!(result, Ok('h'));

        let data = crate::from_source("> hello").unwrap();
        let result = char::deserialize(data.view());
        assert_eq!(
            result,
            Err(marked::DeserializeError::new_invalid_value(
                Mark::new(0, 0),
                "one-character string".into(),
                None
            ))
        );
    }

    #[test]
    fn test_str() {
        let data = crate::from_source("> hello").unwrap();
        let result = <&str>::deserialize(data.view());
        assert_eq!(result, Ok("hello"));

        let data = crate::from_source("hello").unwrap();
        let result = <&str>::deserialize(data.view());
        assert_eq!(
            result,
            Err(marked::DeserializeError::new_invalid_type(
                Mark::new(0, 0),
                NodeType::Raw,
                &[
                    NodeType::String,
                    NodeType::Tagged,
                    NodeType::Anchor,
                    NodeType::Document
                ] as &[_]
            ))
        );
    }

    #[test]
    fn test_option() {
        let data = crate::from_source("= Some: 42").unwrap();
        let result = Option::<u8>::deserialize(data.view());
        assert_eq!(result, Ok(Some(42)));

        let data = crate::from_source("Some").unwrap();
        let result = Option::<u8>::deserialize(data.view());
        assert_eq!(
            result,
            Err(marked::DeserializeError::new_invalid_value(
                Mark::new(0, 0),
                "optional value".into(),
                Some(Box::new(marked::DeserializeError::new_invalid_type(
                    Mark::new(0, 0),
                    NodeType::Raw,
                    &[NodeType::Tagged, NodeType::Anchor, NodeType::Document] as &[_]
                )))
            ))
        );
    }

    #[test]
    fn test_seq() {
        let data = crate::from_source("[0, 2, 67]").unwrap();
        let result = Vec::<u8>::deserialize(data.view());
        assert_eq!(result, Ok(vec![0, 2, 67]));

        let data = crate::from_source("[0, 2, 457]").unwrap();
        let result = Vec::<u8>::deserialize(data.view());
        assert_eq!(
            result,
            Err(marked::DeserializeError::new_invalid_value(
                Mark::new(0, 7),
                "integer in the range from 0 to 2^8 - 1".into(),
                None
            ))
        );
    }

    #[test]
    fn test_tuple() {
        let data = crate::from_source("[2, 67]").unwrap();
        let result = <(u8, u8)>::deserialize(data.view());
        assert_eq!(result, Ok((2, 67)));

        let data = crate::from_source("[0, 2, 457]").unwrap();
        let result = <(u8, u8)>::deserialize(data.view());
        assert_eq!(
            result,
            Err(marked::DeserializeError::new_invalid_length(
                Mark::new(0, 0),
                3
            ))
        );
    }

    #[test]
    fn test_map() {
        let data = crate::from_source("first: 42\nsecond: 15").unwrap();
        let result = HashMap::<String, i32>::deserialize(data.view());
        assert_eq!(
            result,
            Ok(HashMap::from([("first".into(), 42), ("second".into(), 15)]))
        );

        let data = crate::from_source(r#"[["first", 42], ["second"]]"#).unwrap();
        let result = HashMap::<String, i32>::deserialize(data.view());
        assert_eq!(
            result,
            Err(marked::DeserializeError::new_invalid_length(
                Mark::new(0, 16),
                1
            ))
        );
    }

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    struct TestStruct {
        field: i32,
    }

    #[test]
    fn test_struct() {
        let data = crate::from_source("field: 42").unwrap();
        let result = TestStruct::deserialize(data.view());
        assert_eq!(result, Ok(TestStruct { field: 42 }));

        let data = crate::from_source("key: \n\tfield: 42\n\tkey: 15").unwrap();
        let view = data.view().map().unwrap().get("key").unwrap();
        let result = TestStruct::deserialize(view);
        assert_eq!(
            result,
            Err(marked::DeserializeError::new_invalid_value(
                Mark::new(1, 1),
                r#"value of type "TestStruct""#.into(),
                Some(Box::new(marked::DeserializeError::new_unknown_key(
                    Mark::new(1, 1),
                    "key".into(),
                    &["field"] as &[_]
                )))
            ))
        );
    }

    #[test]
    fn test_enum() {
        let data = crate::from_source("= Ok: 42").unwrap();
        let result = Result::<u8, u8>::deserialize(data.view());
        assert_eq!(result, Ok(Ok(42)));

        let data = crate::from_source("key: 15").unwrap();
        let view = data.view().map().unwrap().get("key").unwrap();
        let result = Result::<u8, u8>::deserialize(view);
        assert_eq!(
            result,
            Err(marked::DeserializeError::new_invalid_value(
                Mark::new(0, 5),
                r#"value of type "Result""#.into(),
                Some(Box::new(marked::DeserializeError::new_unknown_raw(
                    Mark::new(0, 5),
                    "15".into(),
                    &["Ok", "Err"] as &[_]
                )))
            ))
        );
    }
}
