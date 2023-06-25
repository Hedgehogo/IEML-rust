use crate::data::mark::Mark;

pub(crate) trait NodeError: std::error::Error {
	fn get_mark(&self) -> Mark;
}

