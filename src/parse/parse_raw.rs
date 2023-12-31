use super::error::marked::ParseResult;
use crate::data::mark::Mark;
use nom::multi::many1_count;
use nom::{character::complete::*, combinator::recognize};

pub(crate) fn parse_raw(mark: Mark) -> impl Fn(&str) -> ParseResult<&str, String> {
    move |input| {
        let (input, result) = recognize(many1_count(none_of("\"\n<>")))(input)?;
        let new_mark = mark + Mark::new(0, result.len());
        Ok((input, (new_mark, result.into())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw() {
        let mark = Mark::new(0, 0);
        {
            let string = "hello".into();
            let result = (Mark::new(0, 5), string);
            assert_eq!(parse_raw(mark)("hello"), Ok(("", result)));
        }
        {
            let string = "hello".into();
            let result = (Mark::new(0, 5), string);
            assert_eq!(parse_raw(mark)("hello\n"), Ok(("\n", result)));
        }
        {
            assert!(parse_raw(mark)("< \n").is_err());
        }
    }
}
