use super::super::super::{
    node::node::StringNode,
    mark::Mark
};
use std::fmt::Debug;

#[derive(Debug, Clone, Eq)]
pub struct StringView<'data> {
    mark: Mark,
    string: &'data StringNode,
}

impl<'data> StringView<'data> {
    pub(in super::super) fn new(mark: Mark, string: &'data StringNode) -> Self {
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
