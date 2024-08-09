//! Type definition [`CustomError`]

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

/// Deserialisation error type containing a child error - the reason.
#[derive(PartialEq, Eq, Debug)]
pub struct CustomError {
    msg: String,
}

impl CustomError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }

    pub fn message(&self) -> &str {
        &self.msg
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self.message())
    }
}

impl Error for CustomError {}

pub mod marked {
    use crate::{data::mark::Mark, error::marked::MarkedError};

    pub type CustomError = MarkedError<super::CustomError>;

    fn parse(value: &str) -> Option<CustomError> {
        let (msg, rest) = value.rsplit_once("\n --> ")?;
        let (line, symbol) = rest.split_once(':')?;
        let line = line.parse::<usize>().ok()?;
        let symbol = symbol.parse::<usize>().ok()?;

        let mark = Mark::new(line, symbol);
        let custom = super::CustomError::new(msg.into());
        Some(CustomError::new(mark, custom))
    }

    impl From<String> for CustomError {
        fn from(value: String) -> Self {
            match parse(value.as_str()) {
                Some(i) => i,
                None => Self::new(Default::default(), super::CustomError::new(value)),
            }
        }
    }
}
