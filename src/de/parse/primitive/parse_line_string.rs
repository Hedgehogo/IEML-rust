use super::super::{
    cursor::Cursor,
    read_source::ReadSource,
    utils::combinator::{cursor::char, parse::match_line},
};
use crate::{
    data::make,
    de::parse::{Error, ErrorKind, LexResult, RateError, Result},
};
use nom::sequence::tuple;

pub(crate) fn lex_line_string<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
) -> LexResult<'input, String> {
    match tuple((char('>'), char(' ')))(cursor) {
        Ok((cursor, _)) => {
            let (cursor, result) = match_line(cursor);
            Ok((cursor, result.input.into()))
        }
        Err(_) => {
            let kind = ErrorKind::FailedDetermineType;
            Err(Error::new_with(cursor.mark, reader.path(), kind))
        }
    }
}

pub(crate) fn parse_line_string<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| match lex_line_string(reader, cursor) {
        Ok((output, string)) => make::string(cursor.mark, output, string)(token),
        Err(error) => Err(RateError::Recoverable((token, error))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::data::mark::Mark;
    use std::path::Path;

    #[test]
    fn test_line_string() {
        let begin_mark = Mark::new(0, 0);
        let path = "test.ieml";
        let reader = Path::new(path);
        assert_eq!(
            lex_line_string(reader, ("> hello", begin_mark).into()),
            Ok((("", Mark::new(0, 7)).into(), "hello".into()))
        );
        assert_eq!(
            lex_line_string(reader, ("> hello\nhello", begin_mark).into()),
            Ok((("\nhello", Mark::new(0, 7)).into(), "hello".into()))
        );
        assert_eq!(
            lex_line_string(reader, (">hello", begin_mark).into()),
            Err(Error::new_with(
                begin_mark,
                path,
                ErrorKind::FailedDetermineType
            ))
        );
    }
}
