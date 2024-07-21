use super::super::super::mark::Mark;
use std::fmt::Debug;

/// Structure for reading Null node data.
#[derive(Debug, Clone, Eq)]
pub struct NullView {
    mark: Mark,
}

impl NullView {
    pub(in super::super) fn new(mark: Mark) -> Self {
        Self { mark }
    }

    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        self.mark
    }
}

impl PartialEq for NullView {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}