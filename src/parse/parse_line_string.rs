use super::error::marked::ParseResult;
use crate::data::mark::Mark;
use nom::bytes::complete::tag;
use nom::multi::many0_count;
use nom::{character::complete::*, combinator::recognize};

pub(crate) fn parse_line_string(mark: Mark) -> impl Fn(&str) -> ParseResult<&str, String> {
    move |input| {
        let (input, _) = tag("> ")(input)?;
        let (input, result) = recognize(many0_count(none_of("\n")))(input)?;
        let new_mark = mark + Mark::new(0, result.len() + 2);
        Ok((input, (new_mark, result.into())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_string() {
        let mark = Mark::new(0, 0);
        {
            let string = "hello".into();
            let result = (Mark::new(0, 7), string);
            assert_eq!(parse_line_string(mark)("> hello"), Ok(("", result)));
        }
        {
            let string = "hello".into();
            let result = (Mark::new(0, 7), string);
            assert_eq!(
                parse_line_string(mark)("> hello\nhello"),
                Ok(("\nhello", result))
            );
        }
        {
            assert!(parse_line_string(mark)(">hello").is_err());
        }
    }
}
