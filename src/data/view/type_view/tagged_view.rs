//! Type definition [`TaggedView`]

use super::super::{
    super::{
        data::Data,
        error::{expected::Expected, marked, CustomError, UnknownTagError},
        mark::Mark,
        name::Name,
        node::tag_node::TaggedNode,
    },
    analyse_anchors::AnalyseAnchors,
    view::View,
};
use serde::de::{self, value::StrDeserializer};
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

    /// Gets a child node if the tag matches at least one of the expected ones.
    ///
    /// Returns the matched tag and node.
    pub fn verified(
        &self,
        expected: impl Into<Expected<&'static str>>,
    ) -> Result<(&'static str, View<'data, A>), marked::UnknownTagError> {
        let expected = Into::<Expected<&'static str>>::into(expected);

        for expected in expected.into_iter() {
            if expected == self.tag().as_str() {
                return Ok((expected, self.view()));
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

    type Variant = View<'data, A>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'data>,
    {
        let tag_deserializer = StrDeserializer::<Self::Error>::new(self.tag().as_str());
        match seed.deserialize(tag_deserializer) {
            Ok(i) => Ok((i, self.view())),
            Err(i) => Err(marked::MarkedError::new(self.mark(), i.data)), 
        }
    }
}
