use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{ParseError, ParseResult, LexResult},
        Error::FailedDetermineType,
    },
};
use crate::data::{make, mark::Mark};
use nom::multi::many1_count;
use nom::{character::complete::*, combinator::recognize};

pub(crate) fn raw<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> LexResult<'input, String> {
    let match_special = many1_count(none_of("\"\n<>"));
    match recognize::<_, _, nom::error::Error<_>, _>(match_special)(cursor.input) {
        Ok((input, result)) => {
            let new_mark = cursor.mark + Mark::new(0, result.len());
            Ok(((input, new_mark).into(), result.into()))
        }
        Err(_) => Err(ParseError::new_with(
            cursor.mark,
            path,
            FailedDetermineType,
        )),
    }
}

pub(crate) fn parse_raw<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> ParseResult<'_, 'input> {
    move |token| match raw(path, cursor) {
        Ok((output, raw)) => make::raw(cursor.mark, output, raw)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        assert_eq!(
            raw(path, ("hello", begin_mark).into()),
            Ok((("", Mark::new(0, 5)).into(), "hello".into()))
        );
        assert_eq!(
            raw(path, ("hello\n", begin_mark).into()),
            Ok((("\n", Mark::new(0, 5)).into(), "hello".into()))
        );
        assert_eq!(
            raw(path, ("< \n", begin_mark).into()),
            Err(ParseError::new_with(
                begin_mark,
                path,
                FailedDetermineType
            ))
        );
    }
}
