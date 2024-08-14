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
    ImpermissibleAnchor,
    ImpermissibleTagged,
    ImpermissibleColon,
    IncompleteString,
    IncompleteShortList,
    IncompleteDocument,
    NonexistentDocument,
}

pub type MakeError = error::Error<ErrorKind>;

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::FailedDetermineType => write!(f, "error: node type couldn't be determined."),
            ErrorKind::ExpectedMapKey => write!(f, "error: expected a map key."),
            ErrorKind::ExpectedListItem => write!(f, "error: expected a list item."),
            ErrorKind::ExpectedTab => write!(f, "error: expected a tab."),
            ErrorKind::ExpectedBlankLine => write!(f, "error: expected a blank line."),
            ErrorKind::ImpermissibleSpace => write!(
                f,
                "error: there is a space at the beginning of the name. Note: You may have meant to indent, you should use tabs for that."
            ),
            ErrorKind::ImpermissibleTab => write!(
                f,
                "error: there is a tab at the beginning of the name. Note: A lower indentation level was expected."
            ),
            ErrorKind::ImpermissibleAnchor => write!(
                f,
                "error: there is a special sequence for anchors (`@`) at the beginning of the name."
            ),
            ErrorKind::ImpermissibleTagged => write!(
                f,
                "error: there is a special sequence for tags (`= `) at the beginning of the name. "
            ),
            ErrorKind::ImpermissibleColon => write!(
                f,
                "error: there is a colon at the ending of the name. "
            ),
            ErrorKind::IncompleteString => write!(f, "error: the string is incomplete."),
            ErrorKind::IncompleteShortList => write!(f, "error: expected `, `, or `]` as a continuation or closure of the short list."),
            ErrorKind::IncompleteDocument => {
                write!(f, "error: there are extra characters at the end of the document.")
            }
            ErrorKind::NonexistentDocument => write!(f, "error: the requested document does not exist."),
        }
    }
}

impl From<name::Error> for ErrorKind {
    fn from(value: name::Error) -> Self {
        match value {
            name::Error::Space => ErrorKind::ImpermissibleSpace,
            name::Error::Tab => ErrorKind::ImpermissibleTab,
            name::Error::AnchorSpecial => ErrorKind::ImpermissibleAnchor,
            name::Error::TaggedSpecial => ErrorKind::ImpermissibleTagged,
            name::Error::Colon => ErrorKind::ImpermissibleColon,
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
    pub type ListResult<'maker, 'input> =
        make::ListResult<'maker, Cursor<'input>, super::ErrorKind>;
    pub type MapResult<'maker, 'input> = make::MapResult<'maker, Cursor<'input>, super::ErrorKind>;

    pub type LexResult<'input, T> = result::Result<(Cursor<'input>, T), Error>;
}
