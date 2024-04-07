use super::super::super::mark::Mark;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Eq)]
pub struct NullNode {
    mark: Mark,
}

impl NullNode {
    pub(super) fn new(mark: Mark) -> Self {
        Self { mark }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }
}

impl PartialEq for NullNode {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}