use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeMapResult, MakeResult, ParseResult},
        Error::{self, ExpectedMapKey, FailedDetermineType},
    },
    name::name,
    parse_node::parse_node,
    read_file::ReadFile,
    utils::combinator::parse::{skip_blank_lines_ln, skip_indent},
};
use crate::data::{
    make::{self, error::MakeErrorReason::Parse},
    name::NameRef,
};

fn key<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
    error: Error,
) -> ParseResult<'input, NameRef<'input>> {
    match name(path, cursor, false) {
        Ok((cursor, (result, _))) => Ok((cursor, result)),
        Err(mut e) => {
            if let Parse(FailedDetermineType) = e.data.reason {
                e.data.reason = Parse(error)
            }
            Err(e)
        }
    }
}

pub(crate) fn parse_map_item<'input, R: ReadFile<'input>>(
    reader: R,
    cursor: Cursor<'input>,
    indent: usize,
    error: Error,
) -> impl FnOnce(make::MapToken) -> MakeMapResult<'_, 'input> {
    move |token| match key(reader.path(), cursor, error) {
        Ok((new_cursor, key)) => {
            let f = parse_node(reader, new_cursor, indent + 1);
            token.add(cursor.mark, key, f)
        }
        Err(error) => Err((token, error)),
    }
}

pub(crate) fn parse_map_one<'input, R: ReadFile<'input>>(
    reader: R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let f = parse_map_item(reader, cursor, indent, FailedDetermineType);
        make::map(cursor.mark, f)(token)
    }
}

pub(crate) fn parse_map<'input, R: ReadFile<'input>>(
    reader: R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let skip_whitespace = |cursor| {
            let (cursor, _) = skip_blank_lines_ln(cursor).ok()?;
            let (cursor, _) = skip_indent(indent)(cursor).ok()?;
            Some(cursor)
        };

        make::map(cursor.mark, |token| {
            let f = parse_map_item(reader.clone(), cursor, indent, FailedDetermineType);
            let result = f(token)?;

            let (mut token, mut cursor) = result;
            loop {
                (token, cursor) = match skip_whitespace(cursor) {
                    Some(cursor) => {
                        parse_map_item(reader.clone(), cursor, indent, ExpectedMapKey)(token)?
                    }
                    None => return Ok((token, cursor)),
                }
            }
        })(token)
    }
}

#[cfg(test)]
mod tests {
    use super::super::error::{
        marked::MakeError,
        Error::{self, FailedDetermineType},
    };
    use crate::data::{mark::Mark};
    use std::path::PathBuf;

    use super::*;

    fn name(i: &str) -> NameRef {
        NameRef::new(i.into()).unwrap()
    }

    #[test]
    fn test_parse_map_one() {
        let begin_mark = Mark::new(0, 0);
        let path = PathBuf::from("test.ieml");
        let path = path.as_path();
        {
            let input = "key: null";
            let data_f = parse_map_one(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 9)).into();
            let result_f = make::map::<_, Error, _>(begin_mark, |token| {
                token.add(
                    Mark::new(0, 0),
                    name("key"),
                    make::null(Mark::new(0, 5), result_output),
                )
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "first: null\n\t\tsecond: null";
            let data_f = parse_map_one(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\tsecond: null", Mark::new(0, 11)).into();
            let result_f = make::map::<_, Error, _>(begin_mark, |token| {
                token.add(
                    Mark::new(0, 0),
                    name("first"),
                    make::null(Mark::new(0, 7), result_output),
                )
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "key:null";
            let data_f = parse_map_one(path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(error_mark, path, FailedDetermineType))
            );
        }
    }

    #[test]
    fn test_parse_map() {
        let begin_mark = Mark::new(0, 0);
        let path = PathBuf::from("test.ieml");
        let path = path.as_path();
        {
            let input = "key: null";
            let data_f = parse_map(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 9)).into();
            let result_f = make::map(begin_mark, |token| {
                token.add(
                    Mark::new(0, 0),
                    name("key"),
                    make::null::<_, Error>(Mark::new(0, 5), result_output),
                )
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "first: null\n\t\tsecond: > hello";
            let data_f = parse_map(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(1, 17)).into();
            let result_f = make::map::<_, Error, _>(begin_mark, |token| {
                let (token, cursor) = token.add(
                    Mark::new(0, 0),
                    name("first"),
                    make::null(Mark::new(0, 7), result_cursor),
                )?;
                let (token, cursor) = token.add(
                    Mark::new(1, 2),
                    name("second"),
                    make::string(Mark::new(1, 10), cursor, "hello"),
                )?;
                Ok((token, cursor))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "first: null\n# hello\n\t\tsecond: > hello";
            let data_f = parse_map(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(2, 17)).into();
            let result_f = make::map::<_, Error, _>(begin_mark, |token| {
                let (token, _) = token.add(
                    Mark::new(0, 0),
                    name("first"),
                    make::null(Mark::new(0, 7), result_cursor),
                )?;
                let (token, _) = token.add(
                    Mark::new(2, 2),
                    name("second"),
                    make::string(Mark::new(2, 10), result_cursor, "hello"),
                )?;
                Ok((token, result_cursor))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "first: null\n# hello\n\t\tsecond:> hello";
            let data_f = parse_map(path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(2, 2);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(error_mark, path, ExpectedMapKey))
            );
        }
        {
            let input = "-null";
            let data_f = parse_map(path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(error_mark, path, FailedDetermineType))
            );
        }
    }
}
