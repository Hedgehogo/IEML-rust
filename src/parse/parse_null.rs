use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult, ParseResult},
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
    file_path: &'input Path,
    cursor: Cursor<'input>,
) -> ParseResult<'input, ()> {
    let match_special = tuple((tag("null"), opt(char(' '))));
    match recognize::<_, _, nom::error::Error<_>, _>(match_special)(cursor.input) {
        Ok((input, result)) => {
            let new_mark = cursor.mark + Mark::new(0, result.len());
            Ok(((input, new_mark).into(), ()))
        }
        Err(_) => Err(MakeError::new_with(
            cursor.mark,
            file_path,
            FailedDetermineType,
        )),
    }
}

pub(crate) fn parse_null<'input, 'path: 'input>(
    file_path: &'path Path,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| match null(file_path, cursor) {
        Ok((output, _)) => make::null(cursor.mark, output)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_null() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        assert_eq!(
            null(file_path, ("null", begin_mark).into()),
            Ok((("", Mark::new(0, 4)).into(), ()))
        );
        assert_eq!(
            null(file_path, ("null ", begin_mark).into()),
            Ok((("", Mark::new(0, 5)).into(), ()))
        );
        assert_eq!(
            null(file_path, ("null# is null", begin_mark).into()),
            Ok((("# is null", Mark::new(0, 4)).into(), ()))
        );
        assert_eq!(
            null(file_path, ("null # is null", begin_mark).into()),
            Ok((("# is null", Mark::new(0, 5)).into(), ()))
        );
        assert_eq!(
            null(file_path, (" null", begin_mark).into()),
            Err(MakeError::new_with(
                begin_mark,
                file_path,
                FailedDetermineType
            ))
        );
    }
}
