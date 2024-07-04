use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult, ParseResult},
        Error::FailedDetermineType,
    },
};
use crate::data::{make, mark::Mark};
use nom::multi::many1_count;
use nom::{character::complete::*, combinator::recognize};

pub(crate) fn raw<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
) -> ParseResult<'input, String> {
    let match_special = many1_count(none_of("\"\n<>"));
    match recognize::<_, _, nom::error::Error<_>, _>(match_special)(cursor.input) {
        Ok((input, result)) => {
            let new_mark = cursor.mark + Mark::new(0, result.len());
            Ok(((input, new_mark).into(), result.into()))
        }
        Err(_) => Err(MakeError::new_with(
            cursor.mark,
            file_path,
            FailedDetermineType,
        )),
    }
}

pub(crate) fn parse_raw<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| match raw(file_path, cursor) {
        Ok((output, raw)) => make::raw(cursor.mark, output, raw)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_raw() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        assert_eq!(
            raw(file_path, ("hello", begin_mark).into()),
            Ok((("", Mark::new(0, 5)).into(), "hello".into()))
        );
        assert_eq!(
            raw(file_path, ("hello\n", begin_mark).into()),
            Ok((("\n", Mark::new(0, 5)).into(), "hello".into()))
        );
        assert_eq!(
            raw(file_path, ("< \n", begin_mark).into()),
            Err(MakeError::new_with(
                begin_mark,
                file_path,
                FailedDetermineType
            ))
        );
    }
}
