use std::{fmt::{Display, Debug}, borrow::Borrow};

/// Error received when trying to create [`Name`] from a non-conforming string.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Error {
    /// The beginning of the line contains a space
    Space,
    /// The beginning of the line contains a tab
    Tab,
}

/// A structure that guarantees that the name contained in it complies with the IEML standard.
#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub struct Name {
    data: String,
}

impl Name {
    /// Creates [`Name`] by checking if the string conforms to the standard.
    ///
    /// # Arguments
    /// * `data` String to be stored in the structure.
    pub fn new(data: String) -> Result<Self, Error> {
        match data.chars().next() {
            Some(' ') => Err(Error::Space),
            Some('\t') => Err(Error::Tab),
            _ => Ok(Self { data }),
        }
    }

    /// Receives internal data
    pub fn as_str(&self) -> &str {
        &self.data
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Borrow<String> for Name {
    fn borrow(&self) -> &String {
        &self.data
    }
}

impl Borrow<str> for Name {
    fn borrow(&self) -> &str {
        &self.data
    }
}

impl<'data> From<NameRef<'data>> for Name {
    fn from(value: NameRef<'data>) -> Self {
        let data = value.data.into();
        Self { data }
    }
}

/// A referenced version of [`Name`], analogous to [`&str`] if we assume that [`Name`] is analogous to [`String`]
#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NameRef<'data> {
    data: &'data str,
}

impl<'data> NameRef<'data> {
    /// Creates [`NameRef`] by checking if the string conforms to the standard.
    ///
    /// # Arguments
    /// * `data` String to be stored in the structure.
    pub fn new(data: &'data str) -> Result<Self, Error> {
        match data.chars().next() {
            Some(' ') => Err(Error::Space),
            Some('\t') => Err(Error::Tab),
            _ => Ok(Self { data }),
        }
    }

    /// Receives internal data
    pub fn as_str(&self) -> &str {
        &self.data
    }
}

impl<'data> Debug for NameRef<'data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<'data> Display for NameRef<'data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<'data> Borrow<str> for NameRef<'data> {
    fn borrow(&self) -> &str {
        &self.data
    }
}

impl<'data> From<&'data Name> for NameRef<'data> {
    fn from(value: &'data Name) -> Self {
        let data = &value.data;
        Self { data }
    }
}
