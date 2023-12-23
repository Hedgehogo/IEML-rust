use super::error::marked::ParseResult;
use crate::data::mark::Mark;
use crate::helpers::blank_lines::match_indent;
use nom::character::complete::*;
use nom::IResult;

fn analyze(input: &str, indent: usize, bytes: usize, mark: Mark) -> IResult<&str, (usize, Mark)> {
    let (input, result) = anychar(input)?;
    match result {
        '\"' => {
            let mark = mark + Mark::new(0, 1);
            Ok((input, (bytes + 1, mark)))
        }
        '\\' => {
            let (input, result) = anychar(input)?;
            match result {
                '\\' | '\"' | 't' | 'n' => {
                    let mark = mark + Mark::new(0, 2);
                    analyze(input, indent, bytes + 1, mark)
                }
                '\n' => {
                    let (input, _) = match_indent(indent)(input)?;
                    let mark = mark + Mark::new(1, indent);
                    analyze(input, indent, bytes, mark)
                }
                i => {
                    let mark = mark + Mark::new(0, 2);
                    analyze(input, indent, bytes + i.len_utf8() + 1, mark)
                }
            }
        }
        '\n' => {
            let (input, _) = match_indent(indent)(input)?;
            let mark = mark + Mark::new(1, indent);
            analyze(input, indent, bytes + 1, mark)
        }
        i => {
            let mark = mark + Mark::new(0, 1);
            analyze(input, indent, bytes + i.len_utf8(), mark)
        }
    }
}

fn parse(input: &str, indent: usize, bytes: usize) -> String {
    let mut result = String::with_capacity(bytes);
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

pub(crate) fn parse_classic_string(
    indent: usize,
    mark: Mark,
) -> impl Fn(&str) -> ParseResult<&str, String> {
    move |input| {
        let (input, _) = char('\"')(input)?;
        let (new_input, (bytes, new_mark)) =
            analyze(input, indent, 0, Mark::new(mark.line, mark.symbol + 1))?;
        let result = parse(input, indent, bytes);
        Ok((new_input, (new_mark, result)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_classic_string() {
        let mark = Mark::new(0, 0);
        {
            let input = r#""hello""#.into();
            let string = "hello".into();
            let result = (Mark::new(0, 7), string);
            assert_eq!(parse_classic_string(2, mark)(input), Ok((r#""#, result)));
        }
        {
            let input = r#""hello"hello"#.into();
            let string = "hello".into();
            let result = (Mark::new(0, 7), string);
            assert_eq!(
                parse_classic_string(2, mark)(input),
                Ok((r#"hello"#, result))
            );
        }
        {
            let input = r#" "hello""#.into();
            assert!(parse_classic_string(2, mark)(input).is_err());
        }
        {
            let input = r#""hello
		world""#
                .into();
            let string = "hello\nworld".into();
            let result = (Mark::new(1, 8), string);
            assert_eq!(parse_classic_string(2, mark)(input), Ok((r#""#, result)));
        }
        {
            let input = r#""hello
			world""#
                .into();
            let string = "hello\n\tworld".into();
            let result = (Mark::new(1, 9), string);
            assert_eq!(parse_classic_string(2, mark)(input), Ok((r#""#, result)));
        }
        {
            let input = r#""hello
	world""#
                .into();
            assert!(parse_classic_string(2, mark)(input).is_err());
        }
        {
            let input = r#""hello \
		world""#
                .into();
            let string = "hello world".into();
            let result = (Mark::new(1, 8), string);
            assert_eq!(parse_classic_string(2, mark)(input), Ok((r#""#, result)));
        }
        {
            let input = r#""hello \"world\"""#.into();
            let string = "hello \"world\"".into();
            let result = (Mark::new(0, 17), string);
            assert_eq!(
                parse_classic_string(2, mark)(input),
                Ok((r#""#, result))
            );
        }
        {
            let input = r#""hello \world""#.into();
            let string = "hello \\world".into();
            let result = (Mark::new(0, 14), string);
            assert_eq!(parse_classic_string(2, mark)(input), Ok((r#""#, result)));
        }
    }
}
