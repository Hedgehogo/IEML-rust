use std::path::Path;

use super::super::{
    cursor::Cursor, name::name, parse_node::parse_node, read_source::ReadSource,
    utils::combinator::cursor::char,
};
use crate::{
    data::{make, name::Name},
    de::parse::{Error, ErrorKind, LexResult, RateError, Result},
};

fn lex_anchor_name<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> LexResult<'input, (Name<&'input str>, bool)> {
    match char('@')(cursor) {
        Ok((cursor, _)) => name(path, cursor, true),
        Err(_) => {
            let kind = ErrorKind::FailedDetermineType;
            Err(Error::new_with(cursor.mark, path, kind))
        }
    }
}

pub(crate) fn parse_anchor<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| match lex_anchor_name(reader.path(), cursor) {
        Ok((output, (name, true))) => {
            let f = parse_node(reader, output, indent);
            make::anchor_creation(cursor.mark, name, f)(token)
        }
        Ok((output, (name, false))) => make::anchor_request(cursor.mark, output, name)(token),
        Err(error) => match error.data.kind {
            make::error::ErrorKind::Parse(ErrorKind::FailedDetermineType) => {
                Err(RateError::Recoverable((token, error)))
            }
            _ => Err(RateError::Unrecoverable((token.error(), error))),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::mark::Mark,
        de::parse::{test_utils::name, ErrorKind},
    };

    use super::*;

    #[test]
    fn test_lex_parse_anchor() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = "@anchor: null\nhello";
            let data_f = parse_anchor(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\nhello", Mark::new(0, 13)).into();
            let result_f = make::anchor_creation::<_, ErrorKind, _, _>(
                begin_mark,
                name("anchor"),
                make::null(Mark::new(0, 9), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "@: null\n\t\thello";
            let data_f = parse_anchor(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\thello", Mark::new(0, 7)).into();
            let result_f = make::anchor_creation::<_, ErrorKind, _, _>(
                begin_mark,
                name(""),
                make::null(Mark::new(0, 3), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = ": null";
            let data_f = parse_anchor(path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::FailedDetermineType
                ))
            );
        }
    }
}
