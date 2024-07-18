pub mod cursor;
pub mod error;
pub mod name;
pub mod parse_alternative;
pub mod parse_complete;
pub mod parse_node;
pub mod parse_scalar;
pub mod primitive;
pub mod read_file;
pub mod utils;

pub use error::{marked::*, ErrorKind};

#[cfg(test)]
mod test_utils {
    use std::{collections::HashMap, path::Path};

    use crate::data::{make, name::NameRef};
    use cursor::Cursor;
    use read_file::{ReadChildFile, ReadResult};

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

    impl<'files, 'path> ReadChildFile for Reader<'files, 'path> {
        fn read_child_file<'maker, F, R>(
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

    pub(super) fn name(i: &str) -> NameRef {
        NameRef::new(i.into()).unwrap()
    }
}
