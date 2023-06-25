use std::fmt::{Debug, Display, Formatter};
use crate::helpers::type_info::{TypeInfo, make_type_info};

#[derive(Debug)]
pub struct Error {
	type_info: Box<dyn TypeInfo>,
	reason: Option<Box<dyn std::error::Error>>,
}

impl Error {
	pub fn new<T: 'static>(reason: Option<Box<dyn std::error::Error>>) -> Error {
		Self{type_info: make_type_info::<T>(), reason}
	}
	
	pub fn get_type_name(&self) -> &'static str {
		self.type_info.get_name()
	}
	
	pub fn get_reason(&self) -> &Option<Box<dyn std::error::Error>> {
		&self.reason
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(i) = &self.reason {
			write!(f, "Failed to convert node to '{}', because:\n{}", self.get_type_name(), i)
		} else {
			write!(f, "Failed to convert node to '{}'.", self.get_type_name())
		}
	}
}

impl std::error::Error for Error {}