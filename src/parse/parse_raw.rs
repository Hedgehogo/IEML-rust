use std::path::Path;

use super::{
    cursor::Cursor,
    utils::combinator::cursor::{none_of, recognize},
};
use super::{Error, ErrorKind, LexResult, Result};
use crate::data::make;
use nom::multi::many1_count;

pub(crate) fn raw<'input>(path: &'input Path, cursor: Cursor<'input>) -> LexResult<'input, String> {
    let match_special = many1_count(none_of("\"\n<>"));
    match recognize(match_special)(cursor) {
        Ok((input, result)) => Ok((input, result.input.into())),
        Err(_) => {
            let reason = ErrorKind::FailedDetermineType;
            Err(Error::new_with(cursor.mark, path, reason))
        }
    }
}

pub(crate) fn parse_raw<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| match raw(path, cursor) {
        Ok((output, raw)) => make::raw(cursor.mark, output, raw)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::mark::Mark;

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
            Err(Error::new_with(
                begin_mark,
                path,
                ErrorKind::FailedDetermineType
            ))
        );
    }
}
