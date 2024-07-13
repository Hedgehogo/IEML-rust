use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::data::make::maker::Token;

use super::cursor::Cursor;

pub type ReadResult<'maker, R> = Result<R, Token<'maker>>;

pub trait ReadFile {
    fn child<'maker, F, R>(&self, token: Token<'maker>, path: &Path, f: F) -> ReadResult<'maker, R>
    where
        F: for<'input> FnOnce(Token<'maker>, &'input Self, Cursor<'input>) -> R;

    fn path(&self) -> &Path;
}

impl ReadFile for Path {
    fn child<'maker, F, R>(&self, token: Token<'maker>, path: &Path, f: F) -> ReadResult<'maker, R>
    where
        F: for<'input> FnOnce(Token<'maker>, &'input Self, Cursor<'input>) -> R,
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

    fn path(&self) -> &Path {
        self
    }
}
