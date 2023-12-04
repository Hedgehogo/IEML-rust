use super::error::marked::ParseResult;
use crate::data::cell::{DataCell, MarkedDataCell};
use crate::data::mark::Mark;
use nom::multi::many1_count;
use nom::{character::complete::*, combinator::recognize};

fn parse_raw(mark: Mark) -> impl Fn(&str) -> ParseResult<&str, MarkedDataCell> {
    move |input| {
        let (input, result) = recognize(many1_count(none_of("\"\n<>")))(input)?;
        let new_mark = mark + Mark::new(0, result.len());
        let cell = MarkedDataCell::new(DataCell::Raw(result.into()), mark);
        Ok((input, (new_mark, cell)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw() {
        let mark = Mark::new(0, 0);
        {
            let cell = MarkedDataCell::new(DataCell::Raw("hello".into()), mark);
            let result = (Mark::new(0, 5), cell);
            assert_eq!(parse_raw(mark)("hello"), Ok(("", result)));
        }
        {
            let cell = MarkedDataCell::new(DataCell::Raw("hello".into()), mark);
            let result = (Mark::new(0, 5), cell);
            assert_eq!(parse_raw(mark)("hello\n"), Ok(("\n", result)));
        }
        {
            assert!(parse_raw(mark)("< \n").is_err());
        }
    }
}
