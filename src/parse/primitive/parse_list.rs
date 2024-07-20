use std::path::Path;

use super::super::{
    cursor::Cursor,
    parse_node::parse_node,
    read_source::ReadSource,
    utils::combinator::{
        cursor::char,
        parse::{skip_blank_line, skip_blank_lines_ln, skip_indent, skip_space},
    },
};
use crate::{
    data::make,
    parse::{Error, ErrorKind, LexResult, ListResult, RateError, Result},
};
use nom::sequence::tuple;

fn special<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
    error_kind: ErrorKind,
) -> LexResult<'input, ()> {
    match tuple((char('-'), skip_space))(cursor) {
        Ok((cursor, _)) => Ok((cursor, ())),
        Err(_) => Err(Error::new_with(cursor.mark, path, error_kind)),
    }
}

fn parse_list_item<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
    error_kind: ErrorKind,
) -> impl FnOnce(make::ListToken) -> ListResult<'_, 'input> {
    move |token| match special(reader.path(), cursor, error_kind) {
        Ok((cursor, _)) => token.add(parse_node(reader, cursor, indent + 1)),
        Err(error) => Err(RateError::Recoverable((token, error))),
    }
}

pub(crate) fn parse_list_one<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| {
        let f = parse_list_item(reader, cursor, indent, ErrorKind::FailedDetermineType);
        make::list(cursor.mark, f)(token)
    }
}

pub(crate) fn parse_list<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| {
        let parse_list_item = |cursor, error_kind| {
            return parse_list_item(reader, cursor, indent, error_kind);
        };

        let skip_whitespace = |cursor| {
            let (cursor, _) = skip_blank_lines_ln(cursor).ok()?;
            let (cursor, _) = skip_indent(indent)(cursor).ok()?;
            Some(cursor)
        };

        make::list(cursor.mark, |token| {
            let f = parse_list_item(cursor, ErrorKind::FailedDetermineType);
            let result = f(token)?;

            let (mut token, mut cursor) = result;
            loop {
                (token, cursor) = match skip_whitespace(cursor) {
                    Some(cursor) => {
                        match parse_list_item(cursor, ErrorKind::ExpectedListItem)(token) {
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
    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_parse_list_one() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = "- null";
            let data_f = parse_list_one(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 6)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                token.add(make::null(Mark::new(0, 2), result_output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "- null\n\t\t- null";
            let data_f = parse_list_one(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\t- null", Mark::new(0, 6)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                token.add(make::null(Mark::new(0, 2), result_output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "-null";
            let data_f = parse_list_one(path, (input, begin_mark).into(), 2);
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
    fn test_parse_list() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = "- null";
            let data_f = parse_list(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 6)).into();
            let result_f = make::list(begin_mark, |token| {
                token.add(make::null::<_, ErrorKind>(Mark::new(0, 2), result_output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "- null\n\t\t- > hello";
            let data_f = parse_list(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(1, 11)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                let (token, output) = token.add(make::null(Mark::new(0, 2), result_output))?;
                let (token, output) = token.add(make::string(Mark::new(1, 4), output, "hello"))?;
                Ok((token, output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "- null\n# hello\n\t\t- > hello";
            let data_f = parse_list(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(2, 11)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                let (token, output) = token.add(make::null(Mark::new(0, 2), result_output))?;
                let (token, output) = token.add(make::string(Mark::new(2, 4), output, "hello"))?;
                Ok((token, output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "- null\n# hello\n\t\t-> hello";
            let data_f = parse_list(path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(2, 2);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::ExpectedListItem
                ))
            );
        }
        {
            let input = "-null";
            let data_f = parse_list(path, (input, begin_mark).into(), 2);
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
