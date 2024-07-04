use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult, ParseResult},
        Error::{ExpectedTab, FailedDetermineType, IncompleteString},
    },
    utils::combinator::{match_indent, skip_blank_line},
};
use crate::data::{make, mark::Mark};
use nom::character::complete::*;

fn analyze<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
    capacity: usize,
) -> ParseResult<'input, usize> {
    let analyze_newline = |input, offset| match match_indent(indent)(input) {
        Ok((input, _)) => {
            let mark = cursor.mark + Mark::new(1, indent);
            analyze(file_path, (input, mark).into(), indent, capacity + offset)
        }
        Err(_) => Err(MakeError::new_with(cursor.mark, file_path, ExpectedTab)),
    };
    let analyze_any = |input, any: char, offset| {
        let mark = cursor.mark + Mark::new(0, 1 + offset);
        let capacity = capacity + any.len_utf8() + offset;
        analyze(file_path, (input, mark).into(), indent, capacity)
    };
    match anychar::<_, nom::error::Error<_>>(cursor.input) {
        Ok((input, result)) => match result {
            '\"' => {
                let mark = cursor.mark + Mark::new(0, 1);
                Ok(((input, mark).into(), capacity + 1))
            }
            '\\' => match anychar::<_, nom::error::Error<_>>(input) {
                Ok((input, result)) => match result {
                    '\\' | '\"' | 't' | 'n' => {
                        let mark = cursor.mark + Mark::new(0, 2);
                        analyze(file_path, (input, mark).into(), indent, capacity + 1)
                    }
                    '\n' => analyze_newline(input, 0),
                    i => analyze_any(input, i, 1),
                },
                Err(_) => {
                    let mark = cursor.mark + Mark::new(0, 1);
                    Err(MakeError::new_with(mark, file_path, IncompleteString))
                }
            },
            '\n' => analyze_newline(input, 1),
            i => analyze_any(input, i, 0),
        },
        Err(_) => Err(MakeError::new_with(cursor.mark, file_path, IncompleteString)),
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
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> ParseResult<'input, String> {
    match char::<_, nom::error::Error<_>>('\"')(cursor.input) {
        Ok((input, _)) => {
            let mark = cursor.mark + Mark::new(0, 1);
            let (cursor, capacity) = analyze(file_path, (input, mark).into(), indent, 0)?;
            let cursor = skip_blank_line(cursor.mark)(cursor.input).into();
            let result = parse(input, indent, capacity);
            Ok((cursor, result))
        }
        Err(_) => Err(MakeError::new_with(cursor.mark, file_path, FailedDetermineType)),
    }
}

pub(crate) fn parse_classic_string<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| match classic_string(file_path, cursor, indent) {
        Ok((output, string)) => make::string(cursor.mark, output, string)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_classic_string() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        {
            let input = r#""hello""#;
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(0, 7)).into(), "hello".into()))
            );
        }
        {
            let input = r#""hello"hello"#;
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Ok((("hello", Mark::new(0, 7)).into(), "hello".into()))
            );
        }
        {
            let input = r#" "hello""#;
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Err(MakeError::new_with(
                    error_mark,
                    file_path,
                    FailedDetermineType
                ))
            );
        }
        {
            let input = r#""hello
		world""#;
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(1, 8)).into(), "hello\nworld".into()))
            );
        }
        {
            let input = r#""hello
			world""#;
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(1, 9)).into(), "hello\n\tworld".into()))
            );
        }
        {
            let input = r#""hello
	world""#;
            let error_mark = Mark::new(0, 6);
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Err(MakeError::new_with(error_mark, file_path, ExpectedTab))
            );
        }
        {
            let input = r#""hello \
		world""#;
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(1, 8)).into(), "hello world".into()))
            );
        }
        {
            let input = r#""hello \"world\"""#;
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(0, 17)).into(), "hello \"world\"".into()))
            );
        }
        {
            let input = r#""hello \world""#;
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(0, 14)).into(), "hello \\world".into()))
            );
        }
        {
            let input = r#""hello \world" # hello"#;
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(0, 22)).into(), "hello \\world".into()))
            );
        }
        {
            let input = r#""hello"#;
            let error_mark = Mark::new(0, 6);
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Err(MakeError::new_with(error_mark, file_path, IncompleteString))
            );
        }
        {
            let input = r#""hello\"#;
            let error_mark = Mark::new(0, 7);
            assert_eq!(
                classic_string(file_path, (input, begin_mark).into(), 2),
                Err(MakeError::new_with(error_mark, file_path, IncompleteString))
            );
        }
    }
}
