use super::super::{
    cursor::Cursor,
    name::name,
    parse_node::parse_node,
    read_source::ReadSource,
    utils::combinator::parse::{skip_blank_line, skip_blank_lines_ln, skip_indent},
};
use crate::{
    data::{make, name::Name},
    de::parse::{ErrorKind, LexResult, MapResult, RateError, Result},
};

fn lex_key<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    error_kind: ErrorKind,
) -> LexResult<'input, Name<&'input str>> {
    match name(reader, cursor, false) {
        Ok((cursor, (result, _))) => Ok((cursor, result)),
        Err(mut e) => {
            if let make::error::ErrorKind::Parse(ErrorKind::FailedDetermineType) = e.data.kind {
                e.data.kind = make::error::ErrorKind::Parse(error_kind)
            }
            Err(e)
        }
    }
}

pub(crate) fn parse_map_item<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
    error: ErrorKind,
) -> impl FnOnce(make::MapToken) -> MapResult<'_, 'input> {
    move |token| match lex_key(reader, cursor, error) {
        Ok((new_cursor, key)) => {
            let f = parse_node(reader, new_cursor, indent + 1);
            token.add(cursor.mark, key, f)
        }
        Err(error) => Err(RateError::Recoverable((token, error))),
    }
}

pub(crate) fn parse_map_one<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| {
        let f = parse_map_item(reader, cursor, indent, ErrorKind::FailedDetermineType);
        make::map(cursor.mark, f)(token)
    }
}

pub(crate) fn parse_map<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| {
        let parse_map_item = |cursor, error_kind| {
            return parse_map_item(reader, cursor, indent, error_kind);
        };

        let skip_whitespace = |cursor| {
            let (cursor, _) = skip_blank_lines_ln(cursor).ok()?;
            let (cursor, _) = skip_indent(indent)(cursor).ok()?;
            Some(cursor)
        };

        make::map(cursor.mark, |token| {
            let f = parse_map_item(cursor, ErrorKind::FailedDetermineType);
            let result = f(token)?;

            let (mut token, mut cursor) = result;
            loop {
                (token, cursor) = match skip_whitespace(cursor) {
                    Some(cursor) => {
                        match parse_map_item(cursor, ErrorKind::ExpectedMapKey)(token) {
                            Ok(i) => i,

                            Err(RateError::Recoverable((token, error))) => {
                                if skip_blank_line(cursor).input.is_empty() {
                                    return Ok((token, cursor));
                                } else {
                                    return Err(RateError::Unrecoverable((token.error(), error)));
                                }
                            }

                            Err(error) => return Err(error),
                        }
                    }
                    None => return Ok((token, cursor)),
                }
            }
        })(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::Error;
    use crate::{data::mark::Mark, de::parse::test_utils::name};
    use std::path::Path;

    #[test]
    fn test_parse_map_one() {
        let begin_mark = Mark::new(0, 0);
        let path = "test.ieml";
        let reader = Path::new(path);
        {
            let input = "key: null";
            let data_f = parse_map_one(reader, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 9)).into();
            let result_f = make::map::<_, ErrorKind, _>(begin_mark, |token| {
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
            let data_f = parse_map_one(reader, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\tsecond: null", Mark::new(0, 11)).into();
            let result_f = make::map::<_, ErrorKind, _>(begin_mark, |token| {
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
            let data_f = parse_map_one(reader, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::FailedDetermineType
                ))
            );
        }
    }

    #[test]
    fn test_parse_map() {
        let begin_mark = Mark::new(0, 0);
        let path = "test.ieml";
        let reader = Path::new(path);
        {
            let input = "key: null";
            let data_f = parse_map(reader, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 9)).into();
            let result_f = make::map(begin_mark, |token| {
                token.add(
                    Mark::new(0, 0),
                    name("key"),
                    make::null::<_, ErrorKind>(Mark::new(0, 5), result_output),
                )
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "first: null\n\t\tsecond: > hello";
            let data_f = parse_map(reader, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(1, 17)).into();
            let result_f = make::map::<_, ErrorKind, _>(begin_mark, |token| {
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
            let data_f = parse_map(reader, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(2, 17)).into();
            let result_f = make::map::<_, ErrorKind, _>(begin_mark, |token| {
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
            let data_f = parse_map(reader, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(2, 2);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(error_mark, path, ErrorKind::ExpectedMapKey))
            );
        }
        {
            let input = "-null";
            let data_f = parse_map(reader, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::FailedDetermineType
                ))
            );
        }
    }
}
