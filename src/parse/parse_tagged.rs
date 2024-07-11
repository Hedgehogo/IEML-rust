use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult},
        Error::FailedDetermineType,
    },
    parse_node::parse_node,
    utils::combinator::{cursor::char, parse::match_name},
};
use crate::data::make;
use nom::sequence::tuple;

pub(crate) fn parse_tagged<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| match tuple((char('='), char(' '), match_name))(cursor) {
        Ok((new_cursor, (_, _, tag))) => {
            make::tagged(cursor.mark, tag, parse_node(file_path, new_cursor, indent))(token)
        }
        Err(_) => {
            let error = MakeError::new_with(cursor.mark, file_path, FailedDetermineType);
            Err((token, error))
        }
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

    #[test]
    fn test_parse_tagged() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        {
            let input = "= tag: null";
            let data_f = parse_tagged(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 11)).into();
            let result_f = make::tagged::<_, Error, _, _>(
                begin_mark,
                "tag",
                make::null(Mark::new(0, 7), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "= : null\n\t\thello";
            let data_f = parse_tagged(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\thello", Mark::new(0, 8)).into();
            let result_f = make::tagged::<_, Error, _, _>(
                begin_mark,
                "",
                make::null(Mark::new(0, 4), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "=tag: null";
            let data_f = parse_tagged(file_path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(
                    error_mark,
                    file_path,
                    FailedDetermineType
                ))
            );
        }
    }
}
