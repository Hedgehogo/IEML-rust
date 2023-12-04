use std::path::PathBuf;

#[derive(PartialEq, Eq, Debug)]
pub enum Reason {
    FailedDetermineType,
    ExpectedMapKey,
    ExpectedListItem,
    ImpermissibleSpace,
    ImpermissibleTab,
    AnchorAlreadyExists,
    IncompleteString,
    IncompleteDocument,
    NonexistentFile,
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParseError {
    pub file_path: PathBuf,
    pub reason: Reason,
}

pub mod marked {
    use crate::data::{error::with_mark::WithMarkError, mark::Mark};
    use nom::IResult;

    pub type ParseError = WithMarkError<super::ParseError>;
    pub type ParseResult<I, O, E = nom::error::Error<I>> = IResult<I, (Mark, O), E>;
}
