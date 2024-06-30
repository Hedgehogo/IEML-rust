use crate::data::mark::Mark;
use nom::error::Error;
use nom::sequence::Tuple;
use nom::{bytes::complete::*, character::complete::*, multi::*, *};

pub fn match_enter(input: &str) -> IResult<&str, ()> {
    Ok((tag("\n")(input)?.0, ()))
}

pub fn skip_enter(mark: Mark) -> impl FnMut(&str) -> IResult<&str, Mark> {
    move |input| match_enter(input).map(|(output, _)| (output, mark + Mark::new(1, 0)))
}

pub fn match_indent(indent: usize) -> impl FnMut(&str) -> IResult<&str, ()> {
    move |input| Ok((many_m_n(indent, indent, tag("\t"))(input)?.0, ()))
}

pub fn skip_indent(indent: usize, mark: Mark) -> impl FnMut(&str) -> IResult<&str, Mark> {
    move |input| match_indent(indent)(input).map(|(input, _)| (input, mark + Mark::new(0, indent)))
}

pub fn match_blank_line(input: &str) -> (&str, usize) {
    let (input, count) = many0_count(one_of::<_, _, nom::error::Error<_>>(" \t"))(input)
        .expect("Internal error in `match_blank_line` function operation.");
    let mut parse_comment = (
        tag::<&str, &str, Error<&str>>("#"),
        one_of("! "),
        many0_count(none_of("\n")),
    );
    match parse_comment.parse(input) {
        Ok((input, (_, _, comment_count))) => (input, count + 2 + comment_count),
        _ => (input, count),
    }
}

pub fn skip_blank_line(mark: Mark) -> impl FnMut(&str) -> (&str, Mark) {
    move |input| {
        let (input, length) = match_blank_line(input);
        (input, Mark::new(mark.line, mark.symbol + length))
    }
}

pub fn skip_blank_lines_ln(mark: Mark) -> impl FnMut(&str) -> IResult<&str, Mark> {
    move |input| {
        fold_many0(
            |input| {
                let (input, _) = match_blank_line(input);
                match_enter(input)
            },
            || mark,
            |mut mark, _| {
                mark.enter();
                mark
            },
        )(input)
    }
}

pub fn match_line(mark: Mark) -> impl FnMut(&str) -> (&str, (&str, Mark)) {
    move |input| {
        let (output, len) = many0_count(none_of::<_, _, nom::error::Error<_>>("\n"))(input)
            .expect("Internal error in `match_line` function operation.");
        let mark = mark + Mark::new(0, len);
        let capacity = input.len() - output.len();
        let (result, _) = input.split_at(capacity);
        (output, (result, mark))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_enter() {
        assert_eq!(match_enter("\nhello"), Ok(("hello", ())));
        assert!(match_enter("hello").is_err());
    }

    #[test]
    fn test_skip_enter() {
        let mark = Mark::new(15, 10);
        assert_eq!(skip_enter(mark)("\nhello"), Ok(("hello", Mark::new(16, 0))));
        assert!(match_enter("hello").is_err());
    }

    #[test]
    fn test_match_indent() {
        let input = "\t\t\thello";
        assert_eq!(match_indent(0)(input), Ok(("\t\t\thello", ())));
        assert_eq!(match_indent(2)(input), Ok(("\thello", ())));
        assert_eq!(match_indent(3)(input), Ok(("hello", ())));
        assert!(match_indent(4)(input).is_err());
    }

    #[test]
    fn test_skip_indent() {
        let input = "\t\t\thello";
        let mark = Mark::new(15, 10);
        assert_eq!(
            skip_indent(0, mark)(input),
            Ok(("\t\t\thello", Mark::new(15, 10)))
        );
        assert_eq!(
            skip_indent(2, mark)(input),
            Ok(("\thello", Mark::new(15, 12)))
        );
        assert_eq!(
            skip_indent(3, mark)(input),
            Ok(("hello", Mark::new(15, 13)))
        );
        assert!(skip_indent(4, mark)(input).is_err());
    }

    #[test]
    fn test_match_blank_line() {
        assert_eq!(match_blank_line("\t  hello"), ("hello", 3));
        assert_eq!(match_blank_line("\t # fg\n b"), ("\n b", 6));
        assert_eq!(match_blank_line("#sadh "), ("#sadh ", 0));
        assert_eq!(match_blank_line(""), ("", 0));
    }

    #[test]
    fn test_skip_blank_line() {
        let mark = Mark::new(15, 10);
        assert_eq!(
            skip_blank_line(mark)("\t  hello"),
            ("hello", Mark::new(15, 13))
        );
        assert_eq!(
            skip_blank_line(mark)("\t # fg\n b"),
            ("\n b", Mark::new(15, 16))
        );
        assert_eq!(
            skip_blank_line(mark)("\t#sadh "),
            ("#sadh ", Mark::new(15, 11))
        );
        assert_eq!(
            skip_blank_line(mark)("#sadh "),
            ("#sadh ", Mark::new(15, 10))
        );
    }

    #[test]
    fn test_skip_blank_lines_ln() {
        let mark = Mark::new(15, 10);
        assert_eq!(
            skip_blank_lines_ln(mark)(" # hello\n\t \t \n world"),
            Ok((" world", Mark::new(17, 0)))
        );
        assert_eq!(
            skip_blank_lines_ln(mark)(" #hello\nhello"),
            Ok((" #hello\nhello", Mark::new(15, 10)))
        );
        assert_eq!(
            skip_blank_lines_ln(mark)(" #hello\nhello"),
            Ok((" #hello\nhello", Mark::new(15, 10)))
        );
    }
}
