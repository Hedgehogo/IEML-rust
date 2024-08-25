//! Type definition [`TaggedView`]

use super::super::{
    super::{
        data::Data,
        error::{expected::Expected, marked, UnknownTagError},
        mark::Mark,
        name::Name,
        node::tag_node::TaggedNode,
    },
    analyse_anchors::AnalyseAnchors,
    view::View,
};
use std::fmt;

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

impl<'data, A: AnalyseAnchors<'data>> fmt::Debug for TaggedView<'data, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TaggedView {{ mark: {:?}, tag: {:?}, view: {:?} }}",
            self.mark,
            self.tag(),
            self.view()
        )
    }
}
