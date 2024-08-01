//! This module is designed to describe structure that guarantee that their contents are names conforming to the IEML standard.
use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
};

/// Error received when trying to create [`Name`] from a non-conforming string.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Error {
    /// The beginning of the line contains a space
    Space,
    /// The beginning of the line contains a tab
    Tab,
}

/// A structure that guarantees that the name contained in it complies with the IEML standard.
#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Name<T> {
    data: T,
}

impl<T: AsRef<str>> Name<T> {
    /// Creates [`Name`] by checking if the string conforms to the standard.
    ///
    /// # Arguments
    /// * `data` String to be stored in the structure.
    pub fn new(data: T) -> Result<Self, Error> {
        match data.as_ref().chars().next() {
            Some(' ') => Err(Error::Space),
            Some('\t') => Err(Error::Tab),
            _ => Ok(Self { data }),
        }
    }

    /// Receives internal data
    pub fn as_str(&self) -> &str {
        self.data.as_ref()
    }
}

impl<T: Debug> Debug for Name<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Display> Display for Name<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T: Borrow<str>> Borrow<str> for Name<T> {
    fn borrow(&self) -> &str {
        self.data.borrow()
    }
}

impl<'data> From<Name<&'data str>> for Name<Box<str>> {
    fn from(value: Name<&'data str>) -> Self {
        let data = value.data.into();
        Self { data }
    }
}

impl<'data> From<&'data Name<Box<str>>> for Name<&'data str> {
    fn from(value: &'data Name<Box<str>>) -> Self {
        let data = &value.data;
        Self { data }
    }
}
