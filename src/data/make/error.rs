use std::fmt::{Display, Formatter};
use std::{error::Error, path::PathBuf};

#[derive(PartialEq, Eq, Debug)]
pub enum MakeErrorReason<E: Error + PartialEq + Eq> {
    AnchorAlreadyExist(String),
    AnchorDoesntExist(String),
    Parse(E),
}

impl<E: Error + PartialEq + Eq> Display for MakeErrorReason<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MakeErrorReason::AnchorAlreadyExist(i) => write!(f, "An attempt was made to take an anchor with the name of an anchor that already exists. Anchor name: {:?}.", i),
            MakeErrorReason::AnchorDoesntExist(i) => write!(f, "There is no requested anchor. Anchor name: {:?}.", i),
            MakeErrorReason::Parse(i) => write!(f, "{i}"),
        }
    }
}

impl<E: Error + PartialEq + Eq> Error for MakeErrorReason<E> {}

impl<E: Error + PartialEq + Eq> From<E> for MakeErrorReason<E> {
    fn from(value: E) -> Self {
        MakeErrorReason::Parse(value)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct MakeError<E: Error + PartialEq + Eq> {
    pub file_path: PathBuf,
    pub reason: MakeErrorReason<E>,
}

impl<E: Error + PartialEq + Eq> MakeError<E> {
    pub fn new(file_path: PathBuf, reason: MakeErrorReason<E>) -> Self {
        Self { file_path, reason }
    }
}

impl<E: Error + PartialEq + Eq> Display for MakeError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.file_path.as_os_str().is_empty() {
            write!(
                f,
                "Failed to parse the data in the file {:?}. {}",
                self.file_path, self.reason
            )
        } else {
            write!(f, "Failed to parse the data. {}", self.reason)
        }
    }
}

impl<E: Error + PartialEq + Eq> Error for MakeError<E> {}

// Add after specializations appear
/*impl<F, I: From<F>> From<ParseError<F>> for ParseError<I> {
    fn from(value: ParseError<F>) -> Self {
        ParseError::new(value.file_path, value.reason.into())
    }
}*/

pub mod marked {
    use super::super::super::error::marked::WithMarkError;

    pub type MakeError<E> = WithMarkError<super::MakeError<E>>;
}
