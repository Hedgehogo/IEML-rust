use super::super::super::{
    cell::data_cell::RawCell,
    mark::Mark
};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Eq)]
pub struct RawView<'data> {
    mark: Mark,
    raw: &'data RawCell,
}

impl<'data> RawView<'data> {
    pub(super) fn new(mark: Mark, raw: &'data RawCell) -> Self {
        Self { mark, raw }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn raw(&self) -> &'data str {
        self.raw.as_str()
    }
}

impl<'data> PartialEq for RawView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.raw.as_str() == other.raw.as_str()
    }
}
