use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidIndexError {
    pub requested_index: usize,
    pub list_size: usize,
}

impl InvalidIndexError {
    pub fn new(requested_index: usize, list_size: usize) -> Self {
        Self {
            requested_index,
            list_size,
        }
    }
}

impl Display for InvalidIndexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "It is not possible to get an item outside the list boundary. Item index: {}. List size: {}.", self.requested_index, self.list_size)
    }
}

impl std::error::Error for InvalidIndexError {}
