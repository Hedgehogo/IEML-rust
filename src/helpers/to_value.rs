use crate::helpers::blank_lines::match_blank_line;
pub use super::number::*;
use nom::{
    *,
    branch::alt,
    combinator::value,
    bytes::complete::*,
};

pub fn to_bool(input: &str) -> Option<bool> {
    let (input, result) = alt::<&str, bool, error::Error<_>, _>((
        value(true, tag("yes")),
        value(false, tag("no"))
    ))(input).ok()?;
    let (input, _) = match_blank_line(input).ok()?;
    input.is_empty().then_some(result)
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