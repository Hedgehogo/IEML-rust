use crate::data::mark::Mark;
use nom::error::Error;
use nom::sequence::Tuple;
use nom::{bytes::complete::*, character::complete::*, multi::*, *};

pub fn match_enter(input: &str) -> IResult<&str, ()> {
    Ok((tag("\n")(input)?.0, ()))
}

pub fn match_indent(indent: usize) -> impl FnMut(&str) -> IResult<&str, ()> {
    move |input| Ok((many_m_n(indent, indent, tag("\t"))(input)?.0, ()))
}

pub fn skip_indent(indent: usize, mark: Mark) -> impl FnMut(&str) -> IResult<&str, Mark> {
    move |input| {
        match_indent(indent)(input).map(|(input, _)| {
            (
                input,
                Mark::new(mark.line, mark.symbol + indent),
            )
        })
    }
}

pub fn match_blank_line(input: &str) -> IResult<&str, usize> {
    let (input, count) = many0_count(one_of(" \t"))(input)?;
    let mut parse = (
        tag::<&str, &str, Error<&str>>("#"),
        one_of("! "),
        many0_count(none_of("\n")),
    );
    Ok(
        match parse.parse(input)
        {
            Ok((input, (_, _, comment_count))) => (input, count + 2 + comment_count),
            _ => (input, count),
        },
    )
}

pub fn skip_blank_line(mark: Mark) -> impl FnMut(&str) -> IResult<&str, Mark> {
    move |input| {
        match_blank_line(input).map(|(input, length)| {
            (
                input,
                Mark::new(mark.line, mark.symbol + length),
            )
        })
    }
}

pub fn skip_blank_lines_ln(mark: Mark) -> impl FnMut(&str) -> IResult<&str, Mark> {
    move |input| {
        fold_many0(
            |input| (match_blank_line, match_enter).parse(input),
            || mark,
            |mut mark, _| {
                mark.enter();
                mark
            },
        )(input)
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
            Ok((
                "\t\t\thello",
                Mark::new(15, 10)
            ))
        );
        assert_eq!(
            skip_indent(2, mark)(input),
            Ok((
                "\thello",
                Mark::new(15, 12)
            ))
        );
        assert_eq!(
            skip_indent(3, mark)(input),
            Ok((
                "hello",
                Mark::new(15, 13)
            ))
        );
        assert!(skip_indent(4, mark)(input).is_err());
    }
    
    #[test]
    fn test_match_blank_line() {
        assert_eq!(match_blank_line("\t  hello"), Ok(("hello", 3)));
        assert_eq!(match_blank_line("\t # fg\n b"), Ok(("\n b", 6)));
        assert_eq!(match_blank_line("#sadh "), Ok(("#sadh ", 0)));
    }
    
    #[test]
    fn test_skip_blank_line() {
        let mark = Mark::new(15, 10);
        assert_eq!(
            skip_blank_line(mark)("\t  hello"),
            Ok((
                "hello",
                Mark::new(15, 13)
            ))
        );
        assert_eq!(
            skip_blank_line(mark)("\t # fg\n b"),
            Ok((
                "\n b",
                Mark::new(15, 16)
            ))
        );
        assert_eq!(
            skip_blank_line(mark)("#sadh "),
            Ok((
                "#sadh ",
                Mark::new(15, 10)
            ))
        );
    }
    
    #[test]
    fn test_skip_blank_lines_ln() {
        let mark = Mark::new(15, 10);
        assert_eq!(
            skip_blank_lines_ln(mark)(" # hello\n\t \t \n world"),
            Ok((
                " world",
                Mark::new(17, 0)
            ))
        );
        assert_eq!(
            skip_blank_lines_ln(mark)(" #hello\nhello"),
            Ok((
                " #hello\nhello",
                Mark::new(15, 10)
            ))
        );
    }
}
