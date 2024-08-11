//! Type definition [`Name`]

use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
};

/// Error received when trying to create [`Name`] from a non-conforming string.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Error {
    /// The beginning of the string contains a space
    Space,
    /// The beginning of the string contains a tab
    Tab,
    /// The beginning of the string contains a special sequence for anchors
    AnchorSpecial,
    /// The beginning of the string contains a special sequence for anchors
    TaggedSpecial,
    /// The ending of the string contains a colon
    Colon,
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
        let mut iter = data.as_ref().chars();
        let data = match iter.next() {
            Some(' ') => Err(Error::Space),
            Some('\t') => Err(Error::Tab),
            Some('@') => Err(Error::AnchorSpecial),
            Some('=') => match iter.next() {
                Some(' ') => Err(Error::TaggedSpecial),
                _ => Ok(data)
            },
            Some(_) => match data.as_ref().ends_with(':') {
                true => Err(Error::Colon),
                _ => Ok(data)
            },
            _ => Ok(data),
        }?;
        Ok(Self { data })
    }
}

impl<T> Name<T> {
    /// Consumes the [`Name`], returning the wrapped value.
    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<'data> Name<&'data str> {
    /// Extracts a string slice containing the entire [`Name`].
    pub fn as_str(&self) -> &'data str {
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

impl<T: AsRef<str>> AsRef<str> for Name<T> {
    fn as_ref(&self) -> &str {
        self.data.as_ref()
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