pub mod cursor;
pub mod error;
pub mod from;
pub(crate) mod name;
pub(crate) mod parse_alternative;
pub(crate) mod parse_complete;
pub(crate) mod parse_node;
pub(crate) mod parse_scalar;
pub(crate) mod primitive;
pub mod read_source;
pub mod utils;

pub use error::{marked::*, ErrorKind};

#[cfg(test)]
mod test_utils {
    use std::{collections::HashMap, path::Path};

    use crate::data::{make, name::Name};
    use cursor::Cursor;
    use read_source::{ReadResult, ReadSource};

    use super::*;

    pub(super) type Files<'path> = HashMap<&'path Path, String>;

    #[derive(Clone)]
    pub(super) struct Reader<'files, 'path> {
        files: &'files Files<'path>,
        path: &'path Path,
    }

    impl<'files, 'path> Reader<'files, 'path> {
        pub(super) fn new(files: &'files Files<'path>, path: &'path Path) -> Self {
            Self { files, path }
        }
    }

    impl<'files, 'path> ReadSource for Reader<'files, 'path> {
        type Child = Self;

        fn read_source<'maker, F, R>(
            &self,
            token: make::Token<'maker>,
            f: F,
        ) -> ReadResult<'maker, R>
        where
            F: for<'input> FnOnce(make::Token<'maker>, Cursor<'input>) -> R,
        {
            match self.files.get(self.path) {
                Some(result) => Ok(f(token, (result.as_str(), Default::default()).into())),
                None => Err(token),
            }
        }

        fn read_child<'maker, F, R>(
            &self,
            token: make::Token<'maker>,
            path: &Path,
            f: F,
        ) -> ReadResult<'maker, R>
        where
            F: for<'input> FnOnce(make::Token<'maker>, &'input Self, Cursor<'input>) -> R,
        {
            match self.files.get_key_value(path) {
                Some((path, result)) => {
                    let cursor = (result.as_str(), Default::default()).into();
                    let reader = Reader::new(self.files, path);
                    Ok(f(token, &reader, cursor))
                }
                None => Err(token),
            }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    pub(super) fn name(i: &str) -> Name<&str> {
        Name::new(i.into()).unwrap()
    }
}
