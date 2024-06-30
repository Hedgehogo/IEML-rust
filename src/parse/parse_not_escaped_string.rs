use std::path::Path;

use super::{
    error::{
        marked::{MakeError, MakeResult, ParseResult},
        Error::{ExpectedTab, FailedDetermineType, IncompleteString},
    },
    utils::combinator::{
        match_enter, match_indent, match_line, skip_blank_line, skip_enter, skip_indent,
    },
};
use crate::data::{make, mark::Mark};
use nom::bytes::complete::tag;

fn analyze<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    indent: usize,
    capacity: usize,
    lines: usize,
    mark: Mark,
) -> ((&'input str, Mark), (usize, usize)) {
    let match_whitespace = match_enter(input).and_then(|(input, _)| match_indent(indent)(input));

    let input = match match_whitespace {
        Ok((input, _)) => input,
        Err(_) => return ((input, mark), (capacity - 1, lines)),
    };

    let (input, (line, mark)) = match_line(mark + Mark::new(1, indent))(input);
    let capacity = capacity + line.len() + 1;
    let lines = lines + 1;
    analyze(file_path, input, indent, capacity, lines, mark)
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
    };
}

pub(crate) fn parse_not_escaped_string<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    indent: usize,
    mark: Mark,
) -> ParseResult<'input, String> {
    let (input, _) = tag::<_, _, nom::error::Error<_>>(">>")(input)
        .map_err(|_| MakeError::new_with(mark, file_path, FailedDetermineType))?;
    let (input, mark) = skip_blank_line(mark + Mark::new(0, 2))(input);
    let (input, mark) = skip_enter(mark)(input)
        .map_err(|_| MakeError::new_with(mark, file_path, IncompleteString))?;
    let (input, mark) = skip_indent(indent, mark)(input)
        .map_err(|_| MakeError::new_with(mark, file_path, ExpectedTab))?;
    let (input, (line, mark)) = match_line(mark)(input);
    let capacity = line.len() + 1;
    let ((output, mark), (capacity, lines)) = analyze(file_path, input, indent, capacity, 1, mark);
    let mut result = String::with_capacity(capacity);
    result.push_str(line);
    parse(input, indent, lines, &mut result);
    Ok(((output, mark), result))
}

pub(crate) fn not_escaped_string<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    indent: usize,
    mark: Mark,
) -> impl FnOnce(&'input mut make::Maker) -> MakeResult<'input> {
    move |maker| {
        let map = |(output, string)| make::string(mark, output, string)(maker);
        parse_not_escaped_string(file_path, input, indent, mark).and_then(map)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_not_escaped_string() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        {
            let input = r#">>
		hello"#;
            assert_eq!(
                parse_not_escaped_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("", Mark::new(1, 7)), "hello".into()))
            );
        }
        {
            let input = r#">>
			hello"#;
            assert_eq!(
                parse_not_escaped_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("", Mark::new(1, 8)), "\thello".into()))
            );
        }
        {
            let input = r#">>
		hello
	hello"#;
            assert_eq!(
                parse_not_escaped_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("\n\thello", Mark::new(1, 7)), "hello".into()))
            );
        }
        {
            let input = r#">>
		hello
		hello
	hello"#;
            assert_eq!(
                parse_not_escaped_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("\n\thello", Mark::new(2, 7)), "hello\nhello".into()))
            );
        }
        {
            let input = r#">> 	# hello
		hello
		hello
	hello"#;
            assert_eq!(
                parse_not_escaped_string(file_path.as_path(), input, 2, begin_mark),
                Ok((("\n\thello", Mark::new(2, 7)), "hello\nhello".into()))
            );
        }
        {
            let input = r#">> 	#hello
		hello
		hello
	hello"#;
            assert_eq!(
                parse_not_escaped_string(file_path.as_path(), input, 2, begin_mark),
                Err(MakeError::new_with(
                    Mark::new(0, 4),
                    file_path.as_path(),
                    IncompleteString
                ))
            );
        }
        {
            let input = r#">>
	hello"#;
            assert_eq!(
                parse_not_escaped_string(file_path.as_path(), input, 2, begin_mark),
                Err(MakeError::new_with(
                    Mark::new(1, 0),
                    file_path.as_path(),
                    ExpectedTab
                ))
            );
        }
    }
}
