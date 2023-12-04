use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub struct InvalidKeyError {
    requested_key: String,
}

impl InvalidKeyError {
    pub fn new(requested_key: String) -> Self {
        Self { requested_key }
    }

    pub fn get_requested_key(&self) -> &String {
        &self.requested_key
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
