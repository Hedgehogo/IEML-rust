use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult, ParseResult},
        Error,
    },
    name::name,
    parse_node::parse_node,
    read_file::ReadFile,
    utils::combinator::cursor::char,
};
use crate::data::{make, name::NameRef};
use nom::sequence::tuple;

fn tag<'input>(path: &'input Path, cursor: Cursor<'input>) -> ParseResult<'input, NameRef<'input>> {
    match tuple((char('='), char(' ')))(cursor) {
        Ok((cursor, _)) => {
            let (cursor, (result, _)) = name(path, cursor, false)?;
            return Ok((cursor, result));
        }
        Err(_) => {
            let error_reason = Error::FailedDetermineType;
            Err(MakeError::new_with(cursor.mark, path, error_reason))
        }
    }
}

pub(crate) fn parse_tagged<'input, R: ReadFile<'input>>(
    reader: R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| match tag(reader.path(), cursor) {
        Ok((new_cursor, tag)) => {
            let f = parse_node(reader, new_cursor, indent);
            make::tagged(cursor.mark, tag, f)(token)
        }
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::error::{
        marked::MakeError,
        Error::{self, FailedDetermineType},
    };
    use crate::data::mark::Mark;
    use std::path::PathBuf;

    use super::*;

    fn name(i: &str) -> NameRef {
        NameRef::new(i.into()).unwrap()
    }

    #[test]
    fn test_parse_tagged() {
        let begin_mark = Mark::new(0, 0);
        let path = PathBuf::from("test.ieml");
        let path = path.as_path();
        {
            let input = "= tag: null";
            let data_f = parse_tagged(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 11)).into();
            let result_f = make::tagged::<_, Error, _, _>(
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
            let result_f = make::tagged::<_, Error, _, _>(
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
                Err(MakeError::new_with(error_mark, path, FailedDetermineType))
            );
        }
    }
}
