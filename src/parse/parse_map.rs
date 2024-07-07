use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeMapResult, MakeResult},
        Error::{self, ExpectedMapKey, FailedDetermineType},
    },
    parse_node::parse_node,
    utils::combinator::{skip_blank_lines_ln, skip_indent},
};
use crate::data::{make, mark::Mark};
use nom::character::complete::{char, none_of, one_of};
use nom::combinator::{cut, not};
use nom::multi::fold_many0;
use nom::sequence::Tuple;

fn match_key<'input>(cursor: Cursor<'input>) -> Option<(&'input str, Cursor<'input>)> {
    let (input, mark) = fold_many0(
        |input| {
            not(|input| (char(':'), one_of(" \n")).parse(input))(input)?;
            cut(none_of::<_, _, nom::error::Error<_>>("\n"))(input)
        },
        || cursor.mark,
        |mark, _| mark + Mark::new(0, 1),
    )(cursor.input)
    .ok()?;

    let bytes = cursor.input.len() - input.len();
    let (result, _) = cursor.input.split_at(bytes);
    let (input, mark) = match input.bytes().nth(1).unwrap() {
        32 => (input.split_at(2).1, mark + Mark::new(0, 2)),
        _ => (input.split_at(1).1, mark + Mark::new(0, 1)),
    };

    Some((result, (input, mark).into()))
}

fn parse_map_item<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
    error: Error,
) -> impl FnOnce(make::MapToken) -> MakeMapResult<'_, 'input> {
    move |token| match match_key(cursor) {
        Some((key, new_cursor)) => {
            let f = parse_node(file_path, new_cursor, indent + 1);
            token.add(cursor.mark, key, f)
        }
        None => {
            let error = MakeError::new_with(cursor.mark, file_path, error);
            Err((token, error))
        }
    }
}

pub(crate) fn parse_map_one<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let f = parse_map_item(file_path, cursor, indent, FailedDetermineType);
        make::map(cursor.mark, f)(token)
    }
}

pub(crate) fn parse_map<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let skip_whitespace = |cursor: Cursor<'input>| {
            let cursor: Cursor = skip_blank_lines_ln(cursor.mark)(cursor.input).ok()?.into();
            let cursor = skip_indent(indent, cursor.mark)(cursor.input).ok()?.into();
            Some(cursor)
        };

        make::map(cursor.mark, |token| {
            let result = parse_map_item(file_path, cursor, indent, FailedDetermineType)(token)?;

            let (mut token, mut cursor) = result;
            loop {
                (token, cursor) = match skip_whitespace(cursor) {
                    Some(cursor) => {
                        parse_map_item(file_path, cursor, indent, ExpectedMapKey)(token)?
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
    use crate::data::mark::Mark;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_map_one() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        {
            let input = "key: null";
            let data_f = parse_map_one(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 9)).into();
            let result_f = make::map::<_, Error, _>(begin_mark, |token| {
                token.add(
                    Mark::new(0, 0),
                    "key",
                    make::null(Mark::new(0, 5), result_output),
                )
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "first: null\n\t\tsecond: null";
            let data_f = parse_map_one(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\tsecond: null", Mark::new(0, 11)).into();
            let result_f = make::map::<_, Error, _>(begin_mark, |token| {
                token.add(
                    Mark::new(0, 0),
                    "first",
                    make::null(Mark::new(0, 7), result_output),
                )
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "key:null";
            let data_f = parse_map_one(file_path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(
                    error_mark,
                    file_path,
                    FailedDetermineType
                ))
            );
        }
    }

    #[test]
    fn test_parse_map() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        {
            let input = "key: null";
            let data_f = parse_map(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 9)).into();
            let result_f = make::map(begin_mark, |token| {
                token.add(
                    Mark::new(0, 0),
                    "key",
                    make::null::<_, Error>(Mark::new(0, 5), result_output),
                )
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "first: null\n\t\tsecond: > hello";
            let data_f = parse_map(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(1, 17)).into();
            let result_f = make::map::<_, Error, _>(begin_mark, |token| {
                let (token, cursor) = token.add(
                    Mark::new(0, 0),
                    "first",
                    make::null(Mark::new(0, 7), result_cursor),
                )?;
                let (token, cursor) = token.add(
                    Mark::new(1, 2),
                    "second",
                    make::string(Mark::new(1, 10), cursor, "hello"),
                )?;
                Ok((token, cursor))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "first: null\n# hello\n\t\tsecond: > hello";
            let data_f = parse_map(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(2, 17)).into();
            let result_f = make::map::<_, Error, _>(begin_mark, |token| {
                let (token, _) = token.add(
                    Mark::new(0, 0),
                    "first",
                    make::null(Mark::new(0, 7), result_cursor),
                )?;
                let (token, _) = token.add(
                    Mark::new(2, 2),
                    "second",
                    make::string(Mark::new(2, 10), result_cursor, "hello"),
                )?;
                Ok((token, result_cursor))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "first: null\n# hello\n\t\tsecond:> hello";
            let data_f = parse_map(file_path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(2, 2);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(error_mark, file_path, ExpectedMapKey))
            );
        }
        {
            let input = "-null";
            let data_f = parse_map(file_path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(
                    error_mark,
                    file_path,
                    FailedDetermineType
                ))
            );
        }
    }
}
