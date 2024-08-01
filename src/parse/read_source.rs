use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::data::make::maker::Token;

use super::cursor::Cursor;

fn read_document(parent: &Path, child: &Path) -> Option<(PathBuf, String)> {
    let read = |path| fs::read_to_string(&path).map(|i| (path, i));

    let mut canonical_path = child.canonicalize().ok()?;
    canonical_path.as_mut_os_string().push(".ieml");
    let relative_path: PathBuf = [parent.parent()?, canonical_path.as_path()]
        .iter()
        .collect();

    read(relative_path).or_else(|_| read(canonical_path)).ok()
}

pub type ReadResult<'maker, R> = Result<R, Token<'maker>>;

pub trait ReadSource {
    type Child: ReadSource + ?Sized;

    fn read_source<'maker, F, T>(&self, token: Token<'maker>, f: F) -> ReadResult<'maker, T>
    where
        F: for<'input> FnOnce(Token<'maker>, Cursor<'input>) -> T;

    fn read_child<'maker, F, T>(
        &self,
        token: Token<'maker>,
        path: &Path,
        f: F,
    ) -> ReadResult<'maker, T>
    where
        F: for<'input> FnOnce(Token<'maker>, &Self::Child, Cursor<'input>) -> T;

    fn path(&self) -> &Path;
}

impl ReadSource for Path {
    type Child = Self;

    fn read_source<'maker, F, T>(&self, token: Token<'maker>, f: F) -> ReadResult<'maker, T>
    where
        F: for<'input> FnOnce(Token<'maker>, Cursor<'input>) -> T,
    {
        match fs::read_to_string(&self) {
            Ok(content) => Ok(f(token, (content.as_str(), Default::default()).into())),
            Err(_) => Err(token),
        }
    }

    fn read_child<'maker, F, T>(
        &self,
        token: Token<'maker>,
        path: &Path,
        f: F,
    ) -> ReadResult<'maker, T>
    where
        F: for<'input> FnOnce(Token<'maker>, &Self::Child, Cursor<'input>) -> T,
    {
        match read_document(self, path) {
            Some((path, content)) => {
                let cursor = (content.as_str(), Default::default()).into();
                Ok(f(token, path.as_path(), cursor))
            }
            None => Err(token),
        }
    }

    fn path(&self) -> &Path {
        self
    }
}

impl ReadSource for str {
    type Child = Path;

    fn read_source<'maker, F, T>(&self, token: Token<'maker>, f: F) -> ReadResult<'maker, T>
    where
        F: for<'input> FnOnce(Token<'maker>, Cursor<'input>) -> T,
    {
        Ok(f(token, (self, Default::default()).into()))
    }

    fn read_child<'maker, F, T>(
        &self,
        token: Token<'maker>,
        path: &Path,
        f: F,
    ) -> ReadResult<'maker, T>
    where
        F: for<'input> FnOnce(Token<'maker>, &Self::Child, Cursor<'input>) -> T,
    {
        path.read_source(token, |token, cursor| f(token, path, cursor))
    }

    fn path(&self) -> &Path {
        Path::new("")
    }
}
