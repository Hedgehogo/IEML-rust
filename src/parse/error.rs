use crate::data::make::error;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    FailedDetermineType,
    ExpectedMapKey,
    ExpectedListItem,
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
            Error::ExpectedMapKey => write!(f, "Expected map key."),
            Error::ExpectedListItem => write!(f, "Expected List Item."),
            Error::ImpermissibleSpace => write!(f, "A space was detected. Perhaps you meant to write a tab as an indentation."),
            Error::ImpermissibleTab => write!(f, "A tab was detected. A lower level of indentation was expected."),
            Error::IncompleteString => write!(f, "An attempt was made to take an anchor with the name of an anchor that already exists."),
            Error::IncompleteDocument => write!(f, "The end of the file has been reached, but the String is not completed."),
            Error::NonexistentFile => write!(f, "The requested file does not exist."),
        }
    }
}

impl std::error::Error for Error {}

pub mod marked {
    use crate::data::{make::error, mark::Mark};
    use nom::IResult;

    pub type MakeError = error::marked::MakeError<super::Error>;
    pub type MakeResult<'input> = error::marked::MakeResult<(&'input str, Mark), super::Error>;

    pub type ParseResult<I, O, E = nom::error::Error<I>> = IResult<I, (Mark, O), E>;
}
