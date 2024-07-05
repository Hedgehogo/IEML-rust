use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult, ParseResult},
        Error::{ExpectedTab, FailedDetermineType, IncompleteString},
    },
    utils::combinator::{
        match_indent, match_line, match_newline, skip_blank_line, skip_indent, skip_newline,
    },
};
use crate::data::{make, mark::Mark};
use nom::bytes::complete::tag;

fn analyze<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
    capacity: usize,
    lines: usize,
) -> (Cursor<'input>, (usize, usize)) {
    let match_whitespace =
        match_newline(cursor.input).and_then(|(input, _)| match_indent(indent)(input).into());

    let input = match match_whitespace {
        Ok((input, _)) => input,
        Err(_) => return (cursor, (capacity - 1, lines)),
    };

    let (input, (line, mark)) = match_line(cursor.mark + Mark::new(1, indent))(input);
    let capacity = capacity + line.len() + 1;
    let lines = lines + 1;
    analyze(file_path, (input, mark).into(), indent, capacity, lines)
}

fn parse<'input>(input: &'input str, indent: usize, lines: usize, result: &mut String) {
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

pub(crate) fn not_escaped_string<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> ParseResult<'input, String> {
    let (input, _) = tag::<_, _, nom::error::Error<_>>(">>")(cursor.input)
        .map_err(|_| MakeError::new_with(cursor.mark, file_path, FailedDetermineType))?;
    let (input, mark) = skip_blank_line(cursor.mark + Mark::new(0, 2))(input);

    let (input, mark) = skip_newline(mark)(input)
        .map_err(|_| MakeError::new_with(mark, file_path, IncompleteString))?;
    let (input, mark) = skip_indent(indent, mark)(input)
        .map_err(|_| MakeError::new_with(mark, file_path, ExpectedTab))?;
    let (input, (line, mark)) = match_line(mark)(input);

    let capacity = line.len() + 1;
    let (cursor, (capacity, lines)) = analyze(file_path, (input, mark).into(), indent, capacity, 1);

    let mut result = String::with_capacity(capacity);
    result.push_str(line);
    parse(input, indent, lines, &mut result);

    Ok((cursor, result))
}

pub(crate) fn parse_not_escaped_string<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token: make::Token| match not_escaped_string(file_path, cursor, indent) {
        Ok((output, string)) => make::string(cursor.mark, output, string)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_not_escaped_string() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        {
            let input = r#">>
		hello"#;
            assert_eq!(
                not_escaped_string(file_path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(1, 7)).into(), "hello".into()))
            );
        }
        {
            let input = r#">>
			hello"#;
            assert_eq!(
                not_escaped_string(file_path, (input, begin_mark).into(), 2),
                Ok((("", Mark::new(1, 8)).into(), "\thello".into()))
            );
        }
        {
            let input = r#">>
		hello
	hello"#;
            assert_eq!(
                not_escaped_string(file_path, (input, begin_mark).into(), 2),
                Ok((("\n\thello", Mark::new(1, 7)).into(), "hello".into()))
            );
        }
        {
            let input = r#">>
		hello
		hello
	hello"#;
            assert_eq!(
                not_escaped_string(file_path, (input, begin_mark).into(), 2),
                Ok((("\n\thello", Mark::new(2, 7)).into(), "hello\nhello".into()))
            );
        }
        {
            let input = r#">> 	# hello
		hello
		hello
	hello"#;
            assert_eq!(
                not_escaped_string(file_path, (input, begin_mark).into(), 2),
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
                not_escaped_string(file_path, (input, begin_mark).into(), 2),
                Err(MakeError::new_with(error_mark, file_path, IncompleteString))
            );
        }
        {
            let input = r#">>
	hello"#;
            let error_mark = Mark::new(1, 0);
            assert_eq!(
                not_escaped_string(file_path, (input, begin_mark).into(), 2),
                Err(MakeError::new_with(error_mark, file_path, ExpectedTab))
            );
        }
    }
}
