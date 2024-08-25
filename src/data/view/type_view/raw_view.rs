//! Type definition [`RawView`]

use super::super::super::{
    error::{
        expected::Expected, marked,
        UnknownRawError,
    },
    mark::Mark,
    node::node::RawNode,
};

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

    /// Checks if the content is equal to one of the expected raw data.
    ///
    /// Returns the index of the matched raw data.
    pub fn verify(
        &self,
        expected: impl Into<Expected<&'static str>>,
    ) -> Result<usize, marked::UnknownRawError> {
        let expected = Into::<Expected<&'static str>>::into(expected);

        for (i, &expected) in expected.iter().enumerate() {
            if expected == self.raw() {
                return Ok(i);
            }
        }

        let error = UnknownRawError::new(self.raw().into(), expected);
        Err(marked::UnknownRawError::new(self.mark, error))
    }
}

impl<'data> PartialEq for RawView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.raw.as_str() == other.raw.as_str()
    }
}
