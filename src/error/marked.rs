//! Type definition [`MarkedError`]

use crate::data::mark::Mark;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

/// Error type containing Mark, which is necessary for most error types to be output to the user.
#[derive(PartialEq, Eq, Debug)]
pub struct MarkedError<E> {
    pub mark: Mark,
    pub data: E,
}

impl<E> MarkedError<E> {
    pub fn new(mark: Mark, data: E) -> Self {
        Self { data, mark }
    }
}

impl<E: Display> Display for MarkedError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n --> {}:{}", self.data, self.mark.line, self.mark.symbol)
    }
}

impl<E: Error> Error for MarkedError<E> {}

// Add after specializations appear
/*impl<F, I: From<F>> From<WithMarkError<F>> for WithMarkError<I> {
    fn from(value: WithMarkError<F>) -> Self {
        WithMarkError::new(value.mark, value.data.into())
    }
}*/
