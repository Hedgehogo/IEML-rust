use super::{cursor::Cursor, read_file::ReadFile};
use crate::{
    data::make,
    parse::{Error, ErrorKind, RateError, Result},
};

pub(crate) type Parse<'input, R> =
    for<'maker> fn(&'input R, Cursor<'input>, usize, make::Token<'maker>) -> Result<'maker, 'input>;

pub(crate) fn parse_alternative<'input, R: ReadFile + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
    iter: impl Iterator<Item = Parse<'input, R>>,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| {
        let mut token = token;
        for f in iter {
            token = match f(reader, cursor, indent, token) {
                Ok(i) => return Ok(i),

                Err(error) => match error {
                    RateError::Recoverable((token, _)) => token,

                    RateError::Unrecoverable(error) => {
                        return Err(RateError::Unrecoverable(error));
                    }
                },
            };
        }

        let error_kind = ErrorKind::FailedDetermineType;
        let error = Error::new_with(cursor.mark, reader.path(), error_kind);
        Err(RateError::Unrecoverable((token.error(), error)))
    }
}
