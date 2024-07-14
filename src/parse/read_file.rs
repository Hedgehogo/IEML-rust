use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::data::make::maker::Token;

use super::cursor::Cursor;

fn read_file(parent: &Path, child: &Path) -> Option<(PathBuf, String)> {
    let read = |path| fs::read_to_string(&path).map(|i| (path, i));

    let canonical_path = child.canonicalize().ok()?;
    let relative_path: PathBuf = [parent.parent()?, canonical_path.as_path()].iter().collect();

    read(relative_path).or_else(|_| read(canonical_path)).ok()
}

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
        let (path, input) = match read_file(self, path) {
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
