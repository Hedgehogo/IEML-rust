use std::{
    fs,
    path::{Path, PathBuf},
};

use super::cursor::Cursor;

pub trait ReadFile<'path>: Clone {
    fn child<F: FnOnce(&Path, Cursor)>(self, path: &Path, f: F) -> Option<F>;

    fn path(&self) -> &'path Path;
}

impl<'path> ReadFile<'path> for &'path Path {
    fn child<F: FnOnce(&Path, Cursor)>(self, path: &Path, f: F) -> Option<F> {
        let read = |path| fs::read_to_string(path).map(|i| (path, i));

        let canonical_path = match path.canonicalize().ok() {
            Some(i) => i,
            None => return Some(f),
        };

        let relative_path: PathBuf = match self.parent() {
            Some(i) => [i, canonical_path.as_path()].iter().collect(),
            None => return Some(f),
        };

        let (path, input) = match read(&relative_path).or_else(|_| read(&canonical_path)).ok() {
            Some(i) => i,
            None => return Some(f),
        };

        f(path, (input.as_str(), Default::default()).into());

        None
    }

    fn path(&self) -> &'path Path {
        self
    }
}
