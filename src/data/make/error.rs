//! Type definition [`Error`]

use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use super::super::name::Name;

/// Type of error kind returned by the combinator system to create Data.
#[derive(PartialEq, Eq, Debug)]
pub enum ErrorKind<E: std::error::Error + PartialEq + Eq> {
    /// In the created Data, an anchor with the same name is created twice at the same depth.
    AnchorAlreadyExist(Name<Box<str>>),
    /// In the created Data, an anchor request is encountered that has not been created.
    AnchorDoesntExist(Name<Box<str>>),
    /// There is a repeating key in the map.
    RepeatedKey,
    /// Error kind added in order to expand the possible error kinds.
    Parse(E),
}

impl<E: std::error::Error + PartialEq + Eq> Display for ErrorKind<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::AnchorAlreadyExist(i) => write!(f, "An attempt was made to take an anchor with the name of an anchor that already exists. Anchor name: {:?}.", i),
            ErrorKind::AnchorDoesntExist(i) => write!(f, "There is no requested anchor. Anchor name: {:?}.", i),
            ErrorKind::RepeatedKey => write!(f,"An attempt was made to add a key to the map that already exists."),
            ErrorKind::Parse(i) => write!(f, "{i}"),
        }
    }
}

impl<E: std::error::Error + PartialEq + Eq> std::error::Error for ErrorKind<E> {}

impl<E: std::error::Error + PartialEq + Eq> From<E> for ErrorKind<E> {
    fn from(value: E) -> Self {
        ErrorKind::Parse(value)
    }
}

/// Error type returned by the combinator system to create Data.
#[derive(PartialEq, Eq, Debug)]
pub struct Error<E: std::error::Error + PartialEq + Eq> {
    /// Path to the document in which the error occurred.
    pub path: PathBuf,
    /// Error kind.
    pub kind: ErrorKind<E>,
}

impl<E: std::error::Error + PartialEq + Eq> Error<E> {
    pub fn new(path: PathBuf, kind: ErrorKind<E>) -> Self {
        Self { path, kind: kind }
    }
}

impl<E: std::error::Error + PartialEq + Eq> Display for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.path.as_os_str().is_empty() {
            write!(
                f,
                "Failed to parse the data in the document {:?}. {}",
                self.path, self.kind
            )
        } else {
            write!(f, "Failed to parse the data. {}", self.kind)
        }
    }
}

impl<E: std::error::Error + PartialEq + Eq> std::error::Error for Error<E> {}

// Add after specializations appear
/*impl<F, I: From<F>> From<ParseError<F>> for ParseError<I> {
    fn from(value: ParseError<F>) -> Self {
        ParseError::new(value.path, value.kind.into())
    }
}*/

/// A type that allows you to distinguish between recoverable and unrecoverable errors
#[derive(PartialEq, Eq, Debug)]
pub enum RateError<E1, E2> {
    Recoverable(E1),
    Unrecoverable(E2),
}

impl<E1, E2> Display for RateError<E1, E2>
where
    E1: Display,
    E2: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RateError::Recoverable(i) => write!(f, "Recoverable: {}", i),
            RateError::Unrecoverable(i) => write!(f, "Unrecoverable: {}", i),
        }
    }
}

impl<E1, E2> std::error::Error for RateError<E1, E2>
where
    E1: std::error::Error,
    E2: std::error::Error,
{
}

pub mod marked {
    use super::super::{
        super::{error::marked::MarkedError, mark::Mark},
        maker::{ListToken, MapToken, Token, UsedToken, ErrorToken},
    };
    use std::path::PathBuf;
    use std::result;

    pub type Error<E> = MarkedError<super::Error<E>>;
    pub type RateError<'maker, E, R> =
        super::RateError<(R, Error<E>), (ErrorToken<'maker>, Error<E>)>;
    pub type Result<'maker, O, E> =
        result::Result<(UsedToken<'maker>, O), RateError<'maker, E, Token<'maker>>>;
    pub type ListResult<'maker, O, E> =
        result::Result<(ListToken<'maker>, O), RateError<'maker, E, ListToken<'maker>>>;
    pub type MapResult<'maker, O, E> =
        result::Result<(MapToken<'maker>, O), RateError<'maker, E, MapToken<'maker>>>;

    pub(in super::super) type ChildResult<'maker, R, E> =
        result::Result<(Token<'maker>, R), RateError<'maker, E, Token<'maker>>>;

    impl<E: std::error::Error + PartialEq + Eq> Error<E> {
        pub fn new_with<P, R>(mark: Mark, path: P, kind: R) -> Self
        where
            P: Into<PathBuf>,
            R: Into<super::ErrorKind<E>>,
        {
            Self::new(mark, super::Error::new(path.into(), kind.into()))
        }
    }
}
