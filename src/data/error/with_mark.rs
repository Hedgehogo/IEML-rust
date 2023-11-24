use super::super::mark::Mark;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct WithMarkError<T> {
    pub mark: Mark,
    pub data: T,
}

impl<T> WithMarkError<T> {
    pub fn new(mark: Mark, data: T) -> Self {
        Self { data, mark }
    }
}

impl<T: Display> Display for WithMarkError<T> {
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
