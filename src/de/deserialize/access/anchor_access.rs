use super::super::{buffer_anchors::BufferAnchors, Error, Result};
use crate::data::{error::CustomError, mark::Mark, view::View};
use serde::de;

pub(crate) struct AnchorAccess<'data, B: BufferAnchors<'data>> {
    mark: Mark,
    view: View<'data, B>,
    name: Option<&'data str>,
}

impl<'data, B: BufferAnchors<'data>> AnchorAccess<'data, B> {
    pub(crate) fn new(mark: Mark, view: View<'data, B>, name: Option<&'data str>) -> Self {
        Self { mark, view, name }
    }

    fn error(self) -> Error {
        let custom_error = CustomError::new("incorrect access to the anchor".into());
        Error::new(self.mark, custom_error.into())
    }
}

macro_rules! impl_error_deserialize {
    ($($name:ident($($a:ident: $t:ty),*)),* $(,)?) => {
        $(
            fn $name<V>(self, $($a: $t,)* _visitor: V) -> Result<V::Value>
            where
                V: de::Visitor<'data>,
            {
                Err(self.error())
            }
        )*
    };
}

impl<'data, B: BufferAnchors<'data>> de::Deserializer<'data> for AnchorAccess<'data, B> {
    type Error = Error;

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'data>,
    {
        B::entry(self.view, self.name, visitor)
    }

    impl_error_deserialize!(
        deserialize_any(),
        deserialize_bool(),
        deserialize_i8(),
        deserialize_i16(),
        deserialize_i32(),
        deserialize_i64(),
        deserialize_i128(),
        deserialize_u8(),
        deserialize_u16(),
        deserialize_u32(),
        deserialize_u64(),
        deserialize_u128(),
        deserialize_f32(),
        deserialize_f64(),
        deserialize_char(),
        deserialize_str(),
        deserialize_string(),
        deserialize_bytes(),
        deserialize_byte_buf(),
        deserialize_unit(),
        deserialize_unit_struct(_name: &'static str),
        deserialize_newtype_struct(_name: &'static str),
        deserialize_seq(),
        deserialize_tuple(_len: usize),
        deserialize_tuple_struct(_name: &'static str, _len: usize),
        deserialize_map(),
        deserialize_struct(_name: &'static str, _fields: &'static [&'static str]),
        deserialize_enum(_name: &'static str, _variants: &'static [&'static str]),
        deserialize_identifier(),
        deserialize_ignored_any(),
    );
}
