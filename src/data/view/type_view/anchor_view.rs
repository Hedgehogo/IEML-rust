//! Type definition [`AnchorView`]

use crate::data::error::deserialize::marked;

use super::super::super::{
    data::Data, error::CustomError, mark::Mark, name::Name, node::anchor_node::AnchorNode,
};
use super::super::{analyse_anchors::AnalyseAnchors, buffer_anchors::BufferAnchors, view::View};
use serde::de;
use std::fmt;

/// Structure for reading Anchor node data.
#[derive(Clone, Eq)]
pub struct AnchorView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data AnchorNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> AnchorView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data AnchorNode,
        data: &'data Data,
        anchor_analyser: A,
    ) -> Self {
        Self {
            mark,
            node,
            data,
            anchor_analyser,
        }
    }

    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        self.mark
    }

    /// Gets the name.
    pub fn name(&self) -> Name<&'data str> {
        (&self.node.name).into()
    }

    /// Asks if the node is an anchor creation.
    pub fn is_creation(&self) -> bool {
        self.node.creation
    }

    /// Gets the view on the child node.
    pub fn view(&self) -> View<'data, A> {
        let node = self.data.get(self.node.node_index);
        View::new(node, self.data, self.anchor_analyser.clone())
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for AnchorView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        match (self.is_creation(), other.is_creation()) {
            (true, true) => self.name() == other.name() && self.view() == other.view(),
            (false, false) => self.name() == other.name(),
            _ => false,
        }
    }
}

impl<'data, A: AnalyseAnchors<'data>> fmt::Debug for AnchorView<'data, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_creation() {
            write!(
                f,
                "AnchorView {{ mark: {:?}, name: {:?}, view: {:?} }}",
                self.mark,
                self.name(),
                self.view()
            )
        } else {
            write!(
                f,
                "AnchorView {{ mark: {:?}, name: {:?} }}",
                self.mark,
                self.name()
            )
        }
    }
}

impl<'data, A: BufferAnchors<'data>> AnchorView<'data, A> {
    pub(in super::super) fn deserializer(self) -> Deserializer<'data, A> {
        Deserializer::new(self)
    }
}

pub(in super::super) struct Deserializer<'data, A: BufferAnchors<'data>> {
    anchor: AnchorView<'data, A>,
}

impl<'data, A: BufferAnchors<'data>> Deserializer<'data, A> {
    fn new(anchor: AnchorView<'data, A>) -> Self {
        Self { anchor }
    }

    fn error(self) -> marked::DeserializeError<CustomError> {
        let custom_error = CustomError::new("incorrect access to the anchor".into());
        marked::DeserializeError::new(self.anchor.mark(), custom_error.into())
    }
}

macro_rules! impl_error_deserialize {
    ($($name:ident($($a:ident: $t:ty),*)),* $(,)?) => {
        $(
            fn $name<V>(self, $($a: $t,)* _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'data>,
            {
                Err(self.error())
            }
        )*
    };
}

impl<'data, A: BufferAnchors<'data>> de::Deserializer<'data> for Deserializer<'data, A> {
    type Error = marked::DeserializeError<CustomError>;

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let anchor_analyser = self.anchor.anchor_analyser.clone();
        let id = anchor_analyser.entry(name, self.anchor)?;
        visitor.visit_u128(id as u128)
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
        deserialize_option(),
        deserialize_unit(),
        deserialize_unit_struct(_name: &'static str),
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
