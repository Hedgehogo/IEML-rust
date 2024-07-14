use std::path::Path;

use super::{
    cursor::Cursor, name::name, parse_node::parse_node, read_file::ReadFile,
    utils::combinator::cursor::char,
};
use super::{Error, ErrorKind, LexResult, Result};
use crate::data::{make, name::NameRef};

fn anchor_name<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> LexResult<'input, (NameRef<'input>, bool)> {
    match char('@')(cursor) {
        Ok((cursor, _)) => name(path, cursor, true),
        Err(_) => {
            let reason = ErrorKind::FailedDetermineType;
            Err(Error::new_with(cursor.mark, path, reason))
        }
    }
}

pub(crate) fn parse_anchor<'input, R: ReadFile + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| match anchor_name(reader.path(), cursor) {
        Ok((output, (name, true))) => {
            let f = parse_node(reader, output, indent);
            make::take_anchor(cursor.mark, name, f)(token)
        }
        Ok((output, (name, false))) => make::get_anchor(cursor.mark, output, name)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::error::ErrorKind::{self, FailedDetermineType};
    use crate::data::mark::Mark;

    use super::*;

    fn name(i: &str) -> NameRef {
        NameRef::new(i.into()).unwrap()
    }

    #[test]
    fn test_parse_anchor() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = "@acnhor: null\nhello";
            let data_f = parse_anchor(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\nhello", Mark::new(0, 13)).into();
            let result_f = make::take_anchor::<_, ErrorKind, _, _>(
                begin_mark,
                name("acnhor"),
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
            let result_f = make::take_anchor::<_, ErrorKind, _, _>(
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
                Err(Error::new_with(error_mark, path, FailedDetermineType))
            );
        }
    }
}
