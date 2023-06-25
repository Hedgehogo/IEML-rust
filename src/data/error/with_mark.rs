use std::fmt::{Debug, Display, Formatter};
use super::{
	node::NodeError,
	super::mark::Mark,
};

#[derive(Clone, Debug)]
pub struct Error<T: std::error::Error> {
	data: T,
	mark: Mark,
}

impl<T: std::error::Error> Error<T> {
	pub fn new(data: T, mark: Mark) -> Self {
		Self{data, mark}
	}
	
	pub fn get_mark(&self) -> Mark {
		self.mark
	}
}

impl<T: std::error::Error> Display for Error<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}: {}", self.mark.line, self.mark.symbol, self.data)
	}
}

impl<T: std::error::Error> std::error::Error for Error<T> {}

impl<T: std::error::Error> NodeError for Error<T> {
	fn get_mark(&self) -> Mark {
		self.mark
	}
}