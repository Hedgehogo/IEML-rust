use std::{
    any::type_name,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug)]
pub struct Error {
    type_name: &'static str,
    reason: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new<T>(reason: Option<Box<dyn std::error::Error>>) -> Error {
        Self { type_name: type_name::<T>(), reason }
    }
    
    pub fn get_type_name(&self) -> &'static str {
        self.type_name
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