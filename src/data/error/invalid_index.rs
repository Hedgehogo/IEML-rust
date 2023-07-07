use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub struct InvalidIndexError {
    requested_index: usize,
    list_size: usize,
}

impl InvalidIndexError {
    pub fn new(requested_index: usize, list_size: usize) -> Self {
        Self { requested_index, list_size }
    }
    
    pub fn get_requested_index(&self) -> usize {
        self.requested_index
    }
    
    pub fn get_list_size(&self) -> usize {
        self.list_size
    }
}

impl Display for InvalidIndexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "It is not possible to get an item outside the list boundary. Item index: {}. List size: {}.", self.requested_index, self.list_size)
    }
}

impl std::error::Error for InvalidIndexError {}