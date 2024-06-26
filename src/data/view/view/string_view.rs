use super::super::super::{
    cell::data_cell::StringCell,
    mark::Mark
};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Eq)]
pub struct StringView<'data> {
    mark: Mark,
    string: &'data StringCell,
}

impl<'data> StringView<'data> {
    pub(super) fn new(mark: Mark, string: &'data StringCell) -> Self {
        Self { mark, string }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn string(&self) -> &'data str {
        self.string.as_str()
    }
}

impl<'data> PartialEq for StringView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.string.as_str() == other.string.as_str()
    }
}
