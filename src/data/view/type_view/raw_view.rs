//! Type definition [`RawView`]

use super::super::super::{
    node::node::RawNode,
    mark::Mark
};
use std::fmt::Debug;

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
}

impl<'data> PartialEq for RawView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.raw.as_str() == other.raw.as_str()
    }
}
