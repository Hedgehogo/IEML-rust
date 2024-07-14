use crate::data::{make::error, name};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    FailedDetermineType,
    ExpectedMapKey,
    ExpectedListItem,
    ExpectedTab,
    ExpectedBlankLine,
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
            Error::ExpectedBlankLine => write!(f, "Expected a blank line."),
            Error::ImpermissibleSpace => write!(
                f,
                "A space was detected. Perhaps you meant to write a tab as an indentation."
            ),
            Error::ImpermissibleTab => write!(
                f,
                "A tab was detected. A lower level of indentation was expected."
            ),
            Error::IncompleteString => write!(f, "The string is incomplete."),
            Error::IncompleteDocument => {
                write!(f, "There are extra characters at the end of the document.")
            }
            Error::NonexistentFile => write!(f, "The requested file does not exist."),
        }
    }
}

impl From<name::Error> for Error {
    fn from(value: name::Error) -> Self {
        match value {
            name::Error::Space => Error::ImpermissibleSpace,
            name::Error::Tab => Error::ImpermissibleTab,
        }
    }
}

impl std::error::Error for Error {}

pub mod marked {
    use super::super::cursor::Cursor;
    use crate::data::make::error::marked;

    pub type ParseError = marked::MakeError<super::Error>;
    pub type ParseResult<'maker, 'input> = marked::MakeResult<'maker, Cursor<'input>, super::Error>;
    pub type ParseListResult<'maker, 'input> = marked::MakeListResult<'maker, Cursor<'input>, super::Error>;
    pub type ParseMapResult<'maker, 'input> = marked::MakeMapResult<'maker, Cursor<'input>, super::Error>;
    
    pub type LexResult<'input, T> = Result<(Cursor<'input>, T), ParseError>;
}
