use std::path::Path;

use super::super::{
    cursor::Cursor, name::name, parse_node::parse_node, read_source::ReadSource,
    utils::combinator::cursor::char,
};
use crate::{
    data::{make, name::NameRef},
    parse::{Error, ErrorKind, LexResult, RateError, Result},
};
use nom::sequence::tuple;

fn lex_tag<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> LexResult<'input, NameRef<'input>> {
    match tuple((char('='), char(' ')))(cursor) {
        Ok((cursor, _)) => {
            let (cursor, (result, _)) = name(path, cursor, false)?;
            return Ok((cursor, result));
        }
        Err(_) => {
            let error_kind = ErrorKind::FailedDetermineType;
            Err(Error::new_with(cursor.mark, path, error_kind))
        }
    }
}

pub(crate) fn parse_tagged<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| match lex_tag(reader.path(), cursor) {
        Ok((new_cursor, tag)) => {
            let f = parse_node(reader, new_cursor, indent);
            make::tagged(cursor.mark, tag, f)(token)
        }
        Err(error) => Err(RateError::Recoverable((token, error))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::mark::Mark,
        parse::{test_utils::name, Error, ErrorKind},
    };

    use super::*;

    #[test]
    fn test_parse_tagged() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = "= tag: null";
            let data_f = parse_tagged(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 11)).into();
            let result_f = make::tagged::<_, ErrorKind, _, _>(
                begin_mark,
                name("tag"),
                make::null(Mark::new(0, 7), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "= : null\n\t\thello";
            let data_f = parse_tagged(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\thello", Mark::new(0, 8)).into();
            let result_f = make::tagged::<_, ErrorKind, _, _>(
                begin_mark,
                name(""),
                make::null(Mark::new(0, 4), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "=tag: null";
            let data_f = parse_tagged(path, (input, begin_mark).into(), 2);
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
