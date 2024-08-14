//! Type definition [`TaggedView`]

use super::super::{
    super::{
        data::Data,
        error::{expected::Expected, marked, CustomError, InvalidLengthError, UnknownTagError},
        mark::Mark,
        name::Name,
        node::tag_node::TaggedNode,
    },
    analyse_anchors::AnalyseAnchors,
    view::View,
};
use serde::{
    de::{self, value::StrDeserializer},
    Deserializer,
};
use std::fmt::{self, Debug, Formatter};

/// Structure for reading Tagged node data.
#[derive(Clone, Eq)]
pub struct TaggedView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data TaggedNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> TaggedView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data TaggedNode,
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

    /// Gets the tag.
    pub fn tag(&self) -> Name<&'data str> {
        (&self.node.tag).into()
    }

    /// Gets the view on the child node.
    pub fn view(&self) -> View<'data, A> {
        let node = self.data.get(self.node.node_index);
        View::new(node, self.data, self.anchor_analyser.clone())
    }

    /// Checks if the tag is equal to one of the expected tags.
    ///
    /// Returns the index of the matched tag.
    pub fn verify(
        &self,
        expected: impl Into<Expected<&'static str>>,
    ) -> Result<usize, marked::UnknownTagError> {
        let expected = Into::<Expected<&'static str>>::into(expected);

        for (i, &expected) in expected.iter().enumerate() {
            if expected == self.tag().as_str() {
                return Ok(i);
            }
        }

        let error = UnknownTagError::new(self.tag().as_str().into(), expected);
        Err(marked::UnknownTagError::new(self.mark, error))
    }

    /// Splits into a tag and a child node.
    pub fn split(self) -> (&'data str, View<'data, A>) {
        (self.tag().as_str(), self.view())
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for TaggedView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        self.tag() == other.tag() && self.view() == other.view()
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for TaggedView<'data, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TaggedView {{ mark: {:?}, tag: {:?}, view: {:?} }}",
            self.mark,
            self.tag(),
            self.view()
        )
    }
}

impl<'data, A: AnalyseAnchors<'data>> de::EnumAccess<'data> for TaggedView<'data, A> {
    type Error = marked::DeserializeError<CustomError>;

    type Variant = TaggedView<'data, A>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'data>,
    {
        let variant_deserializer = StrDeserializer::<Self::Error>::new(self.tag().as_str());
        match seed.deserialize(variant_deserializer) {
            Ok(i) => Ok((i, self)),
            Err(i) => Err(marked::MarkedError::new(self.mark(), i.data)),
        }
    }
}

impl<'data, A: AnalyseAnchors<'data>> de::VariantAccess<'data> for TaggedView<'data, A> {
    type Error = marked::DeserializeError<CustomError>;

    fn unit_variant(self) -> Result<(), Self::Error> {
        let list = self.view().list()?;
        if !list.is_empty() {
            let invalid_length = InvalidLengthError::new(list.len());
            let error = marked::DeserializeError::new(list.mark(), invalid_length.into());
            Err(error)
        } else {
            Ok(())
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'data>,
    {
        seed.deserialize(self.view())
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        self.view().deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'data>,
    {
        let map = self.view().map()?;
        visitor.visit_map(map.access())
    }
}
