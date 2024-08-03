use super::combinator::parse::skip_blank_line;
pub use super::number::*;
use nom::{bytes::complete::*, combinator::value, *};

pub fn to_bool(input: &str) -> Option<bool> {
    let (input, result) = value(true, tag::<_, _, error::Error<_>>("yes"))
        .or(value(false, tag("no")))
        .parse(input)
        .ok()?;
    let cursor = skip_blank_line((input, Default::default()).into());
    cursor.input.is_empty().then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bool() {
        assert_eq!(to_bool("yes"), Some(true));
        assert_eq!(to_bool("no  \t "), Some(false));
        assert_eq!(to_bool("yes # hello"), Some(true));
        assert_eq!(to_bool("nok"), None);
    }
}
