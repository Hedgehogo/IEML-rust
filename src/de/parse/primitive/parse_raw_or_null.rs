use std::path::Path;

use super::super::{
    cursor::Cursor,
    utils::combinator::{
        cursor::{char, none_of, recognize},
        parse::skip_blank_line,
    },
};
use crate::{
    data::make,
    de::parse::{Error, ErrorKind, LexResult, RateError, Result},
};
use nom::{combinator::eof, multi::many1_count, sequence::tuple};

pub(crate) fn lex_raw_or_null<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> LexResult<'input, Cursor<'input>> {
    let match_special = many1_count(none_of("\"\n<>"));
    match recognize(match_special)(cursor) {
        Ok((input, result)) => Ok((input, result)),
        Err(_) => {
            let kind = ErrorKind::FailedDetermineType;
            Err(Error::new_with(cursor.mark, path, kind))
        }
    }
}

pub(crate) fn parse_raw_or_null<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| match lex_raw_or_null(path, cursor) {
        Ok((output, result)) => {
            let null = tuple((char('n'), char('u'), char('l'), char('l')));
            let blank_line = |cursor| Ok((skip_blank_line(cursor), ()));
            match tuple((null, blank_line, eof))(result) {
                Ok(_) => make::null(cursor.mark, output)(token),
                Err(_) => make::raw(cursor.mark, output, result.input)(token),
            }
        }
        Err(error) => Err(RateError::Recoverable((token, error))),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_raw_or_null() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        assert_eq!(
            lex_raw_or_null(path, ("hello", begin_mark).into()),
            Ok((("", Mark::new(0, 5)).into(), ("hello", begin_mark).into()))
        );
        assert_eq!(
            lex_raw_or_null(path, ("hello\n", begin_mark).into()),
            Ok((("\n", Mark::new(0, 5)).into(), ("hello", begin_mark).into()))
        );
        assert_eq!(
            lex_raw_or_null(path, ("< \n", begin_mark).into()),
            Err(Error::new_with(
                begin_mark,
                path,
                ErrorKind::FailedDetermineType
            ))
        );
    }
}
