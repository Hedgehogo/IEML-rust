use std::{
    fmt::{Debug, Display, Formatter},
    error::Error,
};
use super::super::mark::Mark;

#[derive(PartialEq, Eq, Debug)]
pub struct WithMarkError<T: Error + PartialEq + Eq> {
    pub data: T,
    pub mark: Mark,
}

impl<T: Error + PartialEq + Eq> WithMarkError<T> {
    pub fn new(data: T, mark: Mark) -> Self {
        Self { data, mark }
    }
}

impl<T: Error + PartialEq + Eq> Display for WithMarkError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.mark.line, self.mark.symbol, self.data)
    }
}

impl<T: Error + PartialEq + Eq> Error for WithMarkError<T> {}