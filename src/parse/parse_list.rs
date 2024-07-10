use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeListResult, MakeResult},
        Error::{self, ExpectedListItem, FailedDetermineType},
    },
    parse_node::parse_node,
    utils::combinator::{skip_blank_lines_ln, skip_indent, match_space},
};
use crate::data::{make, mark::Mark};
use nom::{character::complete::char, combinator::recognize, sequence::tuple};

fn skip_special<'input>(cursor: Cursor<'input>) -> Option<Cursor<'input>> {
    let (input, result) = recognize(tuple((char('-'), match_space)))(cursor.input).ok()?;
    let mark = cursor.mark + Mark::new(0, result.len());
    Some((input, mark).into())
}

fn parse_list_item<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
    error: Error,
) -> impl FnOnce(make::ListToken) -> MakeListResult<'_, 'input> {
    move |token| match skip_special(cursor) {
        Some(cursor) => token.add(parse_node(file_path, cursor, indent + 1)),
        None => {
            let error = MakeError::new_with(cursor.mark, file_path, error);
            Err((token, error))
        }
    }
}

pub(crate) fn parse_list_one<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let f = parse_list_item(file_path, cursor, indent, FailedDetermineType);
        make::list(cursor.mark, f)(token)
    }
}

pub(crate) fn parse_list<'input>(
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

        make::list(cursor.mark, |token| {
            let result = parse_list_item(file_path, cursor, indent, FailedDetermineType)(token)?;

            let (mut token, mut cursor) = result;
            loop {
                (token, cursor) = match skip_whitespace(cursor) {
                    Some(cursor) => {
                        parse_list_item(file_path, cursor, indent, ExpectedListItem)(token)?
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
    fn test_parse_list_one() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        {
            let input = "- null";
            let data_f = parse_list_one(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 6)).into();
            let result_f = make::list::<_, Error, _>(begin_mark, |token| {
                token.add(make::null(Mark::new(0, 2), result_output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "- null\n\t\t- null";
            let data_f = parse_list_one(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\t- null", Mark::new(0, 6)).into();
            let result_f = make::list::<_, Error, _>(begin_mark, |token| {
                token.add(make::null(Mark::new(0, 2), result_output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "-null";
            let data_f = parse_list_one(file_path, (input, begin_mark).into(), 2);
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
    fn test_parse_list() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        {
            let input = "- null";
            let data_f = parse_list(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 6)).into();
            let result_f = make::list(begin_mark, |token| {
                token.add(make::null::<_, Error>(Mark::new(0, 2), result_output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "- null\n\t\t- > hello";
            let data_f = parse_list(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(1, 11)).into();
            let result_f = make::list::<_, Error, _>(begin_mark, |token| {
                let (token, cursor) = token.add(make::null(Mark::new(0, 2), result_cursor))?;
                let (token, cursor) = token.add(make::string(Mark::new(1, 4), cursor, "hello"))?;
                Ok((token, cursor))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "- null\n# hello\n\t\t- > hello";
            let data_f = parse_list(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(2, 11)).into();
            let result_f = make::list::<_, Error, _>(begin_mark, |token| {
                let (token, _) = token.add(make::null(Mark::new(0, 2), result_cursor))?;
                let (token, _) =
                    token.add(make::string(Mark::new(2, 4), result_cursor, "hello"))?;
                Ok((token, result_cursor))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "- null\n# hello\n\t\t-> hello";
            let data_f = parse_list(file_path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(2, 2);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(error_mark, file_path, ExpectedListItem))
            );
        }
        {
            let input = "-null";
            let data_f = parse_list(file_path, (input, begin_mark).into(), 2);
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
