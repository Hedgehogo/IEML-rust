use std::path::Path;

use super::error::{
    marked::{MakeError, MakeResult, ParseResult},
    Error::{ExpectedTab, FailedDetermineType, IncompleteString},
};
use super::utils::combinator::match_indent;
use crate::data::{make, mark::Mark};
use nom::character::complete::*;

fn analyze<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    indent: usize,
    capacity: usize,
    mark: Mark,
) -> ParseResult<'input, usize> {
    let analyze_newline = |input, offset| match match_indent(indent)(input) {
        Ok((input, _)) => {
            let mark = mark + Mark::new(1, indent);
            analyze(file_path, input, indent, capacity + offset, mark)
        }
        Err(_) => Err(MakeError::new_with(mark, file_path, ExpectedTab)),
    };
    let analyze_any = |input, any: char, offset| {
        let mark = mark + Mark::new(0, 1 + offset);
        let capacity = capacity + any.len_utf8() + offset;
        analyze(file_path, input, indent, capacity, mark)
    };
    match anychar::<_, nom::error::Error<_>>(input) {
        Ok((input, result)) => match result {
            '\"' => {
                let mark = mark + Mark::new(0, 1);
                Ok(((input, mark), capacity + 1))
            }
            '\\' => match anychar::<_, nom::error::Error<_>>(input) {
                Ok((input, result)) => match result {
                    '\\' | '\"' | 't' | 'n' => {
                        let mark = mark + Mark::new(0, 2);
                        analyze(file_path, input, indent, capacity + 1, mark)
                    }
                    '\n' => analyze_newline(input, 0),
                    i => analyze_any(input, i, 1),
                },
                Err(_) => Err(MakeError::new_with(mark, file_path, IncompleteString)),
            },
            '\n' => analyze_newline(input, 1),
            i => analyze_any(input, i, 0),
        },
        Err(_) => Err(MakeError::new_with(mark, file_path, IncompleteString)),
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
                i @ ('\\' | '\"' | 't' | 'n') => result.push(i),
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

pub(crate) fn classic_string<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    indent: usize,
    mark: Mark,
) -> ParseResult<'input, String> {
    match char::<_, nom::error::Error<_>>('\"')(input) {
        Ok((input, _)) => {
            let new_mark = Mark::new(mark.line, mark.symbol + 1);
            let ((new_input, new_mark), capacity) = analyze(file_path, input, indent, 0, new_mark)?;
            let result = parse(input, indent, capacity);
            Ok(((new_input, new_mark), result))
        }
        Err(_) => Err(MakeError::new_with(mark, file_path, FailedDetermineType)),
    }
}

pub(crate) fn parse_classic_string<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    indent: usize,
    mark: Mark,
) -> impl FnOnce(&'input mut make::Maker) -> MakeResult<'input> {
    move |maker| {
        let map = |(output, string)| make::string(mark, output, string)(maker);
        classic_string(file_path, input, indent, mark).and_then(map)
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
        {
            let input = r#""hello""#.into();
            assert_eq!(
                classic_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("", Mark::new(0, 7)), "hello".into()))
            );
        }
        {
            let input = r#""hello"hello"#.into();
            assert_eq!(
                classic_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("hello", Mark::new(0, 7)), "hello".into()))
            );
        }
        {
            let input = r#" "hello""#.into();
            assert_eq!(
                classic_string(file_path.as_path(), input, 2, begin_mark),
                Err(MakeError::new_with(
                    begin_mark,
                    file_path.clone(),
                    FailedDetermineType
                ))
            );
        }
        {
            let input = r#""hello
		world""#
                .into();
            assert_eq!(
                classic_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("", Mark::new(1, 8)), "hello\nworld".into()))
            );
        }
        {
            let input = r#""hello
			world""#
                .into();
            assert_eq!(
                classic_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("", Mark::new(1, 9)), "hello\n\tworld".into()))
            );
        }
        {
            let input = r#""hello
	world""#
                .into();
            assert_eq!(
                classic_string(file_path.as_path(), input, 2, begin_mark),
                Err(MakeError::new_with(Mark::new(0, 6), file_path.clone(), ExpectedTab))
            );
        }
        {
            let input = r#""hello \
		world""#
                .into();
            assert_eq!(
                classic_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("", Mark::new(1, 8)), "hello world".into()))
            );
        }
        {
            let input = r#""hello \"world\"""#.into();
            assert_eq!(
                classic_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("", Mark::new(0, 17)), "hello \"world\"".into()))
            );
        }
        {
            let input = r#""hello \world""#.into();
            assert_eq!(
                classic_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("", Mark::new(0, 14)), "hello \\world".into()))
            );
        }
    }
}
