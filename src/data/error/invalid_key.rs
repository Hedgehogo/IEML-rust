use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidKeyError {
    pub requested_key: String,
}

impl InvalidKeyError {
    pub fn new(requested_key: String) -> Self {
        Self { requested_key }
    }
}

impl Display for InvalidKeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A key named '{}' does not exist in the map.",
            self.requested_key
        )
    }
}

impl std::error::Error for InvalidKeyError {}
