use crate::data::make::error;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    FailedDetermineType,
    ExpectedMapKey,
    ExpectedListItem,
    ExpectedTab,
    ImpermissibleSpace,
    ImpermissibleTab,
    IncompleteString,
    IncompleteDocument,
    NonexistentFile,
}

pub type MakeError = error::MakeError<Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FailedDetermineType => write!(f, "Node type couldn't be determined."),
            Error::ExpectedMapKey => write!(f, "Expected a map key."),
            Error::ExpectedListItem => write!(f, "Expected a list item."),
            Error::ExpectedTab => write!(f, "Expected a tab."),
            Error::ImpermissibleSpace => write!(f, "A space was detected. Perhaps you meant to write a tab as an indentation."),
            Error::ImpermissibleTab => write!(f, "A tab was detected. A lower level of indentation was expected."),
            Error::IncompleteString => write!(f, "The end of the file has been reached, but the String is not completed."),
            Error::IncompleteDocument => write!(f, "There are extra characters at the end of the document."),
            Error::NonexistentFile => write!(f, "The requested file does not exist."),
        }
    }
}

impl std::error::Error for Error {}

pub mod marked {
    use crate::data::{make::error, mark::Mark};

    pub type MakeError = error::marked::MakeError<super::Error>;
    pub type MakeResult<'input> = error::marked::MakeResult<(&'input str, Mark), super::Error>;

    pub type ParseResult<'input, T> = Result<((&'input str, Mark), T), MakeError>;
}
