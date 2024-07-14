use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult, LexResult},
        Error::FailedDetermineType,
    },
};
use crate::data::{make, mark::Mark};
use nom::{
    bytes::complete::*,
    character::complete::*,
    combinator::{opt, recognize},
    sequence::tuple,
};

pub(crate) fn null<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> LexResult<'input, ()> {
    let match_special = tuple((tag("null"), opt(char(' '))));
    match recognize::<_, _, nom::error::Error<_>, _>(match_special)(cursor.input) {
        Ok((input, result)) => {
            let new_mark = cursor.mark + Mark::new(0, result.len());
            Ok(((input, new_mark).into(), ()))
        }
        Err(_) => Err(MakeError::new_with(
            cursor.mark,
            path,
            FailedDetermineType,
        )),
    }
}

pub(crate) fn parse_null<'input, 'path: 'input>(
    path: &'path Path,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| match null(path, cursor) {
        Ok((output, _)) => make::null(cursor.mark, output)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        assert_eq!(
            null(path, ("null", begin_mark).into()),
            Ok((("", Mark::new(0, 4)).into(), ()))
        );
        assert_eq!(
            null(path, ("null ", begin_mark).into()),
            Ok((("", Mark::new(0, 5)).into(), ()))
        );
        assert_eq!(
            null(path, ("null# is null", begin_mark).into()),
            Ok((("# is null", Mark::new(0, 4)).into(), ()))
        );
        assert_eq!(
            null(path, ("null # is null", begin_mark).into()),
            Ok((("# is null", Mark::new(0, 5)).into(), ()))
        );
        assert_eq!(
            null(path, (" null", begin_mark).into()),
            Err(MakeError::new_with(
                begin_mark,
                path,
                FailedDetermineType
            ))
        );
    }
}
