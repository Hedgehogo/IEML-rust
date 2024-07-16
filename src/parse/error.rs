use crate::data::{make::error, name};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub enum ErrorKind {
    FailedDetermineType,
    ExpectedMapKey,
    ExpectedListItem,
    ExpectedTab,
    ExpectedBlankLine,
    ImpermissibleSpace,
    ImpermissibleTab,
    IncompleteString,
    IncompleteShortList,
    IncompleteDocument,
    NonexistentFile,
}

pub type MakeError = error::Error<ErrorKind>;

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::FailedDetermineType => write!(f, "Node type couldn't be determined."),
            ErrorKind::ExpectedMapKey => write!(f, "Expected a map key."),
            ErrorKind::ExpectedListItem => write!(f, "Expected a list item."),
            ErrorKind::ExpectedTab => write!(f, "Expected a tab."),
            ErrorKind::ExpectedBlankLine => write!(f, "Expected a blank line."),
            ErrorKind::ImpermissibleSpace => write!(
                f,
                "A space was detected. Perhaps you meant to write a tab as an indentation."
            ),
            ErrorKind::ImpermissibleTab => write!(
                f,
                "A tab was detected. A lower level of indentation was expected."
            ),
            ErrorKind::IncompleteString => write!(f, "The string is incomplete."),
            ErrorKind::IncompleteShortList => write!(f, "Expected `, `, or `]` as a continuation or closure of the short list."),
            ErrorKind::IncompleteDocument => {
                write!(f, "There are extra characters at the end of the document.")
            }
            ErrorKind::NonexistentFile => write!(f, "The requested file does not exist."),
        }
    }
}

impl From<name::Error> for ErrorKind {
    fn from(value: name::Error) -> Self {
        match value {
            name::Error::Space => ErrorKind::ImpermissibleSpace,
            name::Error::Tab => ErrorKind::ImpermissibleTab,
        }
    }
}

impl std::error::Error for ErrorKind {}

pub mod marked {
    use super::super::cursor::Cursor;
    use crate::data::make;
    use std::result;

    pub use crate::data::make::RateError;

    pub type Error = make::Error<super::ErrorKind>;
    pub type Result<'maker, 'input> = make::Result<'maker, Cursor<'input>, super::ErrorKind>;
    pub type ListResult<'maker, 'input> = make::ListResult<'maker, Cursor<'input>, super::ErrorKind>;
    pub type MapResult<'maker, 'input> = make::MapResult<'maker, Cursor<'input>, super::ErrorKind>;
    
    pub type LexResult<'input, T> = result::Result<(Cursor<'input>, T), Error>;
}
