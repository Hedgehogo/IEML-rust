use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult},
        Error::{ExpectedListItem, FailedDetermineType},
    },
    parse_node::parse_node,
    utils::combinator::{skip_blank_lines_ln, skip_indent},
};
use crate::data::{make, mark::Mark};
use nom::character::complete::char;

fn skip_special<'input>(cursor: Cursor<'input>) -> Option<Cursor<'input>> {
    match char::<_, nom::error::Error<_>>('-')(cursor.input) {
        Ok((input, _)) => {
            if let Ok((input, _)) = char::<_, nom::error::Error<_>>(' ')(input) {
                return Some((input, cursor.mark + Mark::new(0, 2)).into());
            }
            if let Ok(_) = char::<_, nom::error::Error<_>>('\n')(input) {
                return Some((input, cursor.mark + Mark::new(0, 1)).into());
            }
            None
        }
        Err(_) => None,
    }
}

pub(crate) fn parse_list_one<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let make_error = |mark, error| MakeError::new_with(mark, file_path, error);

        make::list(cursor.mark, |token| match skip_special(cursor) {
            Some(cursor) => token.add(parse_node(file_path, cursor, indent + 1)),
            None => Err((token, make_error(cursor.mark, FailedDetermineType))),
        })(token)
    }
}

pub(crate) fn parse_list<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let make_error = |mark, error| MakeError::new_with(mark, file_path, error);

        let skip_whitespace = |cursor: Cursor<'input>| {
            let cursor: Cursor = skip_blank_lines_ln(cursor.mark)(cursor.input).ok()?.into();
            let cursor = skip_indent(indent, cursor.mark)(cursor.input).ok()?.into();
            Some(cursor)
        };

        make::list(cursor.mark, |token| {
            let (mut token, mut cursor) = match skip_special(cursor) {
                Some(cursor) => token.add(parse_node(file_path, cursor, indent + 1))?,
                None => return Err((token, make_error(cursor.mark, FailedDetermineType))),
            };

            loop {
                (token, cursor) = {
                    let cursor = match skip_whitespace(cursor) {
                        Some(i) => i,
                        None => return Ok((token, cursor)),
                    };
                    match skip_special(cursor) {
                        Some(cursor) => token.add(parse_node(file_path, cursor, indent + 1))?,
                        None => return Err((token, make_error(cursor.mark, ExpectedListItem))),
                    }
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
