use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult, ParseResult},
        Error::{ExpectedTab, FailedDetermineType, IncompleteString},
    },
    utils::combinator::{
        cursor::{anychar, char},
        parse::{skip_blank_line, skip_indent},
    },
};
use crate::data::make;

fn analyze<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
    capacity: usize,
) -> ParseResult<'input, usize> {
    let analyze_newline = |cursor, offset| match skip_indent(indent)(cursor) {
        Ok((cursor, _)) => analyze(path, cursor, indent, capacity + offset),
        Err(_) => Err(MakeError::new_with(cursor.mark, path, ExpectedTab)),
    };

    let analyze_any = |cursor, any: char, offset| {
        let capacity = capacity + any.len_utf8() + offset;
        analyze(path, cursor, indent, capacity)
    };

    match anychar(cursor) {
        Ok((cursor, result)) => match result {
            '\"' => Ok((cursor, capacity + 1)),

            '\\' => match anychar(cursor) {
                Ok((cursor, result)) => match result {
                    '\\' | '\"' | 't' | 'n' => analyze(path, cursor, indent, capacity + 1),

                    '\n' => analyze_newline(cursor, 0),

                    i => analyze_any(cursor, i, 1),
                },

                Err(_) => Err(MakeError::new_with(
                    cursor.mark,
                    path,
                    IncompleteString,
                )),
            },

            '\n' => analyze_newline(cursor, 1),

            i => analyze_any(cursor, i, 0),
        },

        Err(_) => Err(MakeError::new_with(
            cursor.mark,
            path,
            IncompleteString,
        )),
    }
}

fn parse(input: &str, indent: usize, capacity: usize) -> String {
    let mut result = String::with_capacity(capacity);
    let mut iter = input.chars();
    loop {
        match iter.next().unwrap() {
            '\"' => break,

            '\n' => {
                result.push('\n');
                for _ in 0..indent {
                    iter.next();
                }
            }

            '\\' => match iter.next().unwrap() {
                '\n' => {
                    for _ in 0..indent {
                        iter.next();
                    }
                }

                't' => result.push('\t'),

                'n' => result.push('\n'),

                i @ ('\\' | '\"') => result.push(i),

                i => {
                    result.push('\\');
                    result.push(i);
                }
            },
            i => result.push(i),
        }
    }
    result
}

pub(crate) fn classic_string<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> ParseResult<'input, String> {
    match char('\"')(cursor) {
        Ok((cursor, _)) => {
            let (output, capacity) = analyze(path, cursor, indent, 0)?;
            let output = skip_blank_line(output);
            let result = parse(cursor.input, indent, capacity);
            Ok((output, result))
        }
        Err(_) => Err(MakeError::new_with(
            cursor.mark,
            path,
            FailedDetermineType,
        )),
    }
}

pub(crate) fn parse_classic_string<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| match classic_string(path, cursor, indent) {
        Ok((output, string)) => make::string(cursor.mark, output, string)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_classic_string() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = r#""hello""#;
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(0, 7)).into(), "hello".into()))
            );
        }
        {
            let input = r#""hello"hello"#;
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Ok((("hello", Mark::new(0, 7)).into(), "hello".into()))
            );
        }
        {
            let input = r#" "hello""#;
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Err(MakeError::new_with(
                    error_mark,
                    path,
                    FailedDetermineType
                ))
            );
        }
        {
            let input = r#""hello
		world""#;
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(1, 8)).into(), "hello\nworld".into()))
            );
        }
        {
            let input = r#""hello
			world""#;
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(1, 9)).into(), "hello\n\tworld".into()))
            );
        }
        {
            let input = r#""hello
	world""#;
            let error_mark = Mark::new(1, 0);
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Err(MakeError::new_with(error_mark, path, ExpectedTab))
            );
        }
        {
            let input = r#""hello \
		world""#;
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(1, 8)).into(), "hello world".into()))
            );
        }
        {
            let input = r#""hello \"world\"""#;
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(0, 17)).into(), "hello \"world\"".into()))
            );
        }
        {
            let input = r#""hello \world""#;
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(0, 14)).into(), "hello \\world".into()))
            );
        }
        {
            let input = r#""hello \world" # hello"#;
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(0, 22)).into(), "hello \\world".into()))
            );
        }
        {
            let input = r#""hello"#;
            let error_mark = Mark::new(0, 6);
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Err(MakeError::new_with(error_mark, path, IncompleteString))
            );
        }
        {
            let input = r#""hello\"#;
            let error_mark = Mark::new(0, 7);
            assert_eq!(
                classic_string(path, (input, begin_mark).into(), 2),
                Err(MakeError::new_with(error_mark, path, IncompleteString))
            );
        }
    }
}
