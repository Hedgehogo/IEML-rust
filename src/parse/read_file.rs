use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::data::make::maker::Token;

use super::cursor::Cursor;

pub type ReadResult<'maker, R> = Result<R, Token<'maker>>;

pub trait ReadFile<'path>: Clone {
    fn child<'maker, F, R>(self, token: Token<'maker>, path: &Path, f: F) -> ReadResult<'maker, R>
    where
        F: FnOnce(Token<'maker>, Self, Cursor) -> R;

    fn path(&self) -> &'path Path;
}

impl<'path> ReadFile<'path> for &'path Path {
    fn child<'maker, F, R>(self, token: Token<'maker>, path: &Path, f: F) -> ReadResult<'maker, R>
    where
        F: FnOnce(Token<'maker>, Self, Cursor) -> R,
    {
        let read = |path| fs::read_to_string(&path).map(|i| (path, i));

        let canonical_path = match path.canonicalize().ok() {
            Some(i) => i,
            None => return Err(token),
        };

        let relative_path: PathBuf = match self.parent() {
            Some(i) => [i, canonical_path.as_path()].iter().collect(),
            None => return Err(token),
        };

        let (path, input) = match read(relative_path).or_else(|_| read(canonical_path)).ok() {
            Some(i) => i,
            None => return Err(token),
        };

        let cursor = (input.as_str(), Default::default()).into();
        Ok(f(token, path.as_path(), cursor))
    }

    fn path(&self) -> &'path Path {
        self
    }
}
