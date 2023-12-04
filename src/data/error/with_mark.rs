use super::super::mark::Mark;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

#[derive(PartialEq, Eq, Debug)]
pub struct WithMarkError<T: Error + PartialEq + Eq> {
    pub mark: Mark,
    pub data: T,
}

impl<T: Error + PartialEq + Eq> WithMarkError<T> {
    pub fn new(mark: Mark, data: T) -> Self {
        Self { data, mark }
    }
}

impl<T: Error + PartialEq + Eq> Display for WithMarkError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.mark.line, self.mark.symbol, self.data)
    }
}

// Add after specializations appear
/*impl<F, I: From<F>> From<WithMarkError<F>> for WithMarkError<I> {
    fn from(value: WithMarkError<F>) -> Self {
        WithMarkError::new(value.mark, value.data.into())
    }
}*/
