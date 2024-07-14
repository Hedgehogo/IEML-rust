use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use super::super::name::Name;

#[derive(PartialEq, Eq, Debug)]
pub enum ErrorKind<E: std::error::Error + PartialEq + Eq> {
    AnchorAlreadyExist(Name),
    AnchorDoesntExist(Name),
    RepeatedKey,
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

#[derive(PartialEq, Eq, Debug)]
pub struct Error<E: std::error::Error + PartialEq + Eq> {
    pub path: PathBuf,
    pub kind: ErrorKind<E>,
}

impl<E: std::error::Error + PartialEq + Eq> Error<E> {
    pub fn new(path: PathBuf, reason: ErrorKind<E>) -> Self {
        Self { path, kind: reason }
    }
}

impl<E: std::error::Error + PartialEq + Eq> Display for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.path.as_os_str().is_empty() {
            write!(
                f,
                "Failed to parse the data in the file {:?}. {}",
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
        ParseError::new(value.path, value.reason.into())
    }
}*/

pub mod marked {
    use super::super::{
        super::{error::marked::WithMarkError, mark::Mark},
        maker::{ListToken, MapToken, Token, UsedToken},
    };
    use std::path::PathBuf;
    use std::result;

    pub type Error<E> = WithMarkError<super::Error<E>>;
    pub type Result<'maker, O, E> =
        result::Result<(UsedToken<'maker>, O), (Token<'maker>, Error<E>)>;
    pub type ListResult<'maker, O, E> =
        result::Result<(ListToken<'maker>, O), (ListToken<'maker>, Error<E>)>;
    pub type MapResult<'maker, O, E> =
        result::Result<(MapToken<'maker>, O), (MapToken<'maker>, Error<E>)>;

    impl<E: std::error::Error + PartialEq + Eq> Error<E> {
        pub fn new_with<P, R>(mark: Mark, path: P, reason: R) -> Self
        where
            P: Into<PathBuf>,
            R: Into<super::ErrorKind<E>>,
        {
            Self::new(mark, super::Error::new(path.into(), reason.into()))
        }
    }
}
