#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ListNode {
    pub(crate) data: Vec<usize>,
}

impl ListNode {
    pub(crate) fn new(data: Vec<usize>) -> Self {
        Self { data }
    }
}
