use super::error::marked::ParseResult;
use crate::data::mark::Mark;
use nom::{
    bytes::complete::*,
    character::complete::*,
    combinator::{opt, recognize},
    sequence::tuple,
};

pub(crate) fn parse_null(mark: Mark) -> impl Fn(&str) -> ParseResult<&str, ()> {
    move |input| {
        let (input, result) = recognize(tuple((tag("null"), opt(char(' ')))))(input)?;
        let new_mark = mark + Mark::new(0, result.len());
        Ok((input, (new_mark, ())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        assert_eq!(
            parse_null(Mark::new(0, 0))("null"),
            Ok(("", (Mark::new(0, 4), ())))
        );
        assert_eq!(
            parse_null(Mark::new(0, 0))("null "),
            Ok(("", (Mark::new(0, 5), ())))
        );
        assert_eq!(
            parse_null(Mark::new(0, 0))("null# is null"),
            Ok(("# is null", (Mark::new(0, 4), ())))
        );
        assert_eq!(
            parse_null(Mark::new(0, 0))("null # is null"),
            Ok(("# is null", (Mark::new(0, 5), ())))
        );
        assert!(parse_null(Mark::new(0, 0))(" null").is_err());
    }
}
