use std::{fmt::Display, borrow::Borrow};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Error {
    Space,
    Tab,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Name {
    data: String,
}

impl Name {
    pub fn new(data: String) -> Result<Self, Error> {
        match data.chars().next() {
            Some(' ') => Err(Error::Space),
            Some('\t') => Err(Error::Tab),
            _ => Ok(Self { data }),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.data
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

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NameRef<'data> {
    data: &'data str,
}

impl<'data> NameRef<'data> {
    pub fn new(data: &'data str) -> Result<Self, Error> {
        match data.chars().next() {
            Some(' ') => Err(Error::Space),
            Some('\t') => Err(Error::Tab),
            _ => Ok(Self { data }),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.data
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
