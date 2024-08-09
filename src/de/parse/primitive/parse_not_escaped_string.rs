use std::path::Path;

use super::super::{
    cursor::Cursor,
    utils::combinator::{
        cursor::char,
        parse::{match_line, skip_blank_line, skip_indent, skip_line_ending},
    },
};
use crate::{
    data::make,
    de::parse::{Error, ErrorKind, LexResult, RateError, Result},
};
use nom::sequence::tuple;

fn analyze(
    cursor: Cursor,
    indent: usize,
    capacity: usize,
    lines: usize,
) -> (Cursor, (usize, usize)) {
    let match_whitespace =
        skip_line_ending(cursor).and_then(|(cursor, _)| skip_indent(indent)(cursor));

    let cursor = match match_whitespace {
        Ok((cursor, _)) => cursor,
        Err(_) => return (cursor, (capacity - 1, lines)),
    };

    let (cursor, line) = match_line(cursor);
    let capacity = capacity + line.input.len() + 1;
    let lines = lines + 1;
    analyze(cursor, indent, capacity, lines)
}

fn parse(input: &str, indent: usize, lines: usize, result: &mut String) {
    let mut input = input;
    for _ in 1..lines {
        let (_, end_input) = input.split_at(indent + 1);
        let end_index = end_input.find('\n').unwrap_or(end_input.len());
        let (line, end_input) = end_input.split_at(end_index);
        input = end_input;
        result.push('\n');
        result.push_str(line);
    }
}

pub(crate) fn lex_not_escaped_string<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> LexResult<'input, String> {
    let (cursor, _) = tuple((char('>'), char('>')))(cursor)
        .map_err(|_| Error::new_with(cursor.mark, path, ErrorKind::FailedDetermineType))?;
    let cursor = skip_blank_line(cursor);

    let (cursor, _) = skip_line_ending(cursor)
        .map_err(|_| Error::new_with(cursor.mark, path, ErrorKind::IncompleteString))?;
    let (cursor, _) = skip_indent(indent)(cursor)
        .map_err(|_| Error::new_with(cursor.mark, path, ErrorKind::ExpectedTab))?;
    let (cursor, line) = match_line(cursor);

    let capacity = line.input.len() + 1;
    let (output, (capacity, lines)) = analyze(cursor, indent, capacity, 1);

    let mut result = String::with_capacity(capacity);
    result.push_str(line.input);
    parse(cursor.input, indent, lines, &mut result);

    Ok((output, result))
}

pub(crate) fn parse_not_escaped_string<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token: make::Token| match lex_not_escaped_string(path, cursor, indent) {
        Ok((output, string)) => make::string(cursor.mark, output, string)(token),
        Err(error) => match error.data.kind {
            make::error::ErrorKind::Parse(ErrorKind::FailedDetermineType) => {
                Err(RateError::Recoverable((token, error)))
            }
            _ => Err(RateError::Unrecoverable((token.error(), error))),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_lex_not_escaped_string() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = r#">>
		hello"#;
            assert_eq!(
                lex_not_escaped_string(path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(1, 7)).into(), "hello".into()))
            );
        }
        {
            let input = r#">>
			hello"#;
            assert_eq!(
                lex_not_escaped_string(path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(1, 8)).into(), "\thello".into()))
            );
        }
        {
            let input = r#">>
		hello
	hello"#;
            assert_eq!(
                lex_not_escaped_string(path, (input, begin_mark).into(), 2),
                Ok((("\n\thello", Mark::new(1, 7)).into(), "hello".into()))
            );
        }
        {
            let input = r#">>
		hello
		hello
	hello"#;
            assert_eq!(
                lex_not_escaped_string(path, (input, begin_mark).into(), 2),
                Ok((("\n\thello", Mark::new(2, 7)).into(), "hello\nhello".into()))
            );
        }
        {
            let input = r#">> 	# hello
		hello
		hello
	hello"#;
            assert_eq!(
                lex_not_escaped_string(path, (input, begin_mark).into(), 2),
                Ok((("\n\thello", Mark::new(2, 7)).into(), "hello\nhello".into()))
            );
        }
        {
            let input = r#">> 	#hello
		hello
		hello
	hello"#;
            let error_mark = Mark::new(0, 4);
            assert_eq!(
                lex_not_escaped_string(path, (input, begin_mark).into(), 2),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::IncompleteString
                ))
            );
        }
        {
            let input = r#">>
	hello"#;
            let error_mark = Mark::new(1, 0);
            assert_eq!(
                lex_not_escaped_string(path, (input, begin_mark).into(), 2),
                Err(Error::new_with(error_mark, path, ErrorKind::ExpectedTab))
            );
        }
    }
}
