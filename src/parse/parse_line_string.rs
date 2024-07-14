use std::path::Path;

use super::{
    cursor::Cursor,
    utils::combinator::{cursor::char, parse::match_line},
};
use super::{Error, ErrorKind, LexResult, Result};
use crate::data::make;
use nom::sequence::tuple;

pub(crate) fn line_string<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> LexResult<'input, String> {
    match tuple((char('>'), char(' ')))(cursor) {
        Ok((cursor, _)) => {
            let (cursor, result) = match_line(cursor);
            Ok((cursor, result.into()))
        }
        Err(_) => {
            let kind = ErrorKind::FailedDetermineType;
            Err(Error::new_with(cursor.mark, path, kind))
        }
    }
}

pub(crate) fn parse_line_string<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| match line_string(path, cursor) {
        Ok((output, string)) => make::string(cursor.mark, output, string)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_line_string() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        assert_eq!(
            line_string(path, ("> hello", begin_mark).into()),
            Ok((("", Mark::new(0, 7)).into(), "hello".into()))
        );
        assert_eq!(
            line_string(path, ("> hello\nhello", begin_mark).into()),
            Ok((("\nhello", Mark::new(0, 7)).into(), "hello".into()))
        );
        assert_eq!(
            line_string(path, (">hello", begin_mark).into()),
            Err(Error::new_with(
                begin_mark,
                path,
                ErrorKind::FailedDetermineType
            ))
        );
    }
}
