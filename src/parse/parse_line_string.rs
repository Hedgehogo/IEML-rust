use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult, ParseResult},
        Error::FailedDetermineType,
    },
    utils::combinator::{char, match_line},
};
use crate::data::make;
use nom::sequence::tuple;

pub(crate) fn line_string<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
) -> ParseResult<'input, String> {
    match tuple((char('>'), char(' ')))(cursor) {
        Ok((cursor, _)) => {
            let (cursor, result) = match_line(cursor);
            Ok((cursor, result.into()))
        }
        Err(_) => Err(MakeError::new_with(
            cursor.mark,
            file_path,
            FailedDetermineType,
        )),
    }
}

pub(crate) fn parse_line_string<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| match line_string(file_path, cursor) {
        Ok((output, string)) => make::string(cursor.mark, output, string)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_line_string() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        assert_eq!(
            line_string(file_path, ("> hello", begin_mark).into()),
            Ok((("", Mark::new(0, 7)).into(), "hello".into()))
        );
        assert_eq!(
            line_string(file_path, ("> hello\nhello", begin_mark).into()),
            Ok((("\nhello", Mark::new(0, 7)).into(), "hello".into()))
        );
        assert_eq!(
            line_string(file_path, (">hello", begin_mark).into()),
            Err(MakeError::new_with(
                begin_mark,
                file_path,
                FailedDetermineType
            ))
        );
    }
}
