use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
	requested_key: String
}

impl Error {
	pub fn new(requested_key: String) -> Self {
		Self{requested_key}
	}
	
	pub fn get_requested_key(&self) -> &String {
		&self.requested_key
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "A key named '{}' does not exist in the map.", self.requested_key)
	}
}

impl std::error::Error for Error {}