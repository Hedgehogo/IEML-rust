use std::path::{Path, PathBuf};

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult, ParseResult},
        Error,
    },
    parse_map::parse_map_item,
    parse_node::parse_node,
    read_file::ReadFile,
    utils::combinator::{
        cursor::char,
        parse::{match_line, skip_blank_lines_ln, skip_indent},
    },
};
use crate::data::make;
use nom::sequence::tuple;

fn path<'input>(path: &'input Path, cursor: Cursor<'input>) -> ParseResult<'input, PathBuf> {
    match tuple((char('<'), char(' ')))(cursor) {
        Ok((cursor, _)) => {
            let (cursor, result) = match_line(cursor);
            return Ok((cursor, PathBuf::from(path)));
        }
        Err(_) => {
            let error_reason = Error::FailedDetermineType;
            Err(MakeError::new_with(cursor.mark, path, error_reason))
        }
    }
}

pub(crate) fn parse_tagged<'input, R: ReadFile<'input>>(
    reader: R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| match path(reader.path(), cursor) {
        Ok((new_cursor, path)) => {
            let mark = cursor.mark;

            let reader_map = reader.clone();
            let acnhors = move |token| {
                let skip_whitespace = |cursor| {
                    let (cursor, _) = skip_blank_lines_ln(cursor).ok()?;
                    let (cursor, _) = skip_indent(indent)(cursor).ok()?;
                    Some(cursor)
                };

                let (mut cursor, mut token) = (new_cursor, token);
                loop {
                    (token, cursor) = match skip_whitespace(cursor) {
                        Some(cursor) => {
                            let reader = reader_map.clone();
                            let error_reason = Error::ExpectedMapKey;
                            parse_map_item(reader, cursor, indent, error_reason)(token)?
                        }
                        None => return Ok((token, cursor)),
                    }
                }
            };

            let f = move |token, reader: R, cursor: Cursor<'_>| {
                let path = reader.path().to_path_buf();
                let f = parse_node(reader, cursor, 0);
                make::file(cursor.mark, path, acnhors, f)(token)
            };

            match reader.child(token, path.as_path(), f) {
                Ok(i) => i,
                Err(token) => {
                    let error_reason = Error::NonexistentFile;
                    let error = MakeError::new_with(cursor.mark, path, error_reason);
                    Err((token, error))
                }
            }
        }
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        error::{
            marked::MakeError,
            Error::{self, FailedDetermineType},
        },
        read_file::{ReadFile, ReadResult},
    };
    use crate::data::{mark::Mark, name::NameRef};
    use std::collections::HashMap;

    use super::*;

    type Files = HashMap<PathBuf, String>;

    #[derive(Clone, Copy)]
    struct Reader<'files, 'path> {
        files: &'files Files,
        path: &'path Path,
    }

    impl<'files, 'path> Reader<'files, 'path> {
        fn new(files: &'files Files, path: &'path Path) -> Self {
            Self { files, path }
        }
    }

    impl<'files, 'path> ReadFile<'path> for Reader<'files, 'path> {
        fn child<'maker, F, R>(
            self,
            token: make::Token<'maker>,
            path: &Path,
            f: F,
        ) -> ReadResult<'maker, R>
        where
            F: FnOnce(make::Token<'maker>, Self, Cursor) -> R,
        {
            match self.files.get(path) {
                Some(result) => {
                    let cursor = (result.as_str(), Default::default()).into();
                    let reader = Reader::new(self.files, path);
                    Ok(f(token, reader, cursor))
                }
                None => Err(token)
            }
        }

        fn path(&self) -> &'path Path {
            self.path
        }
    }

    fn name(i: &str) -> NameRef {
        NameRef::new(i.into()).unwrap()
    }

    #[test]
    fn test_parse_file() {
        let begin_mark = Mark::new(0, 0);
        let path = PathBuf::from("test.ieml");
        let path = path.as_path();
    }
}
