use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult, ParseResult},
        Error,
    },
    name::name,
    parse_node::parse_node,
    utils::combinator::cursor::char,
};
use crate::data::make;

fn anchor_name<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
) -> ParseResult<'input, (&'input str, bool)> {
    match char('@')(cursor) {
        Ok((cursor, _)) => name(file_path, cursor, true),
        Err(_) => {
            let reason = Error::FailedDetermineType;
            Err(MakeError::new_with(cursor.mark, file_path, reason))
        }
    }
}

pub(crate) fn parse_anchor<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| match anchor_name(file_path, cursor) {
        Ok((output, (name, true))) => {
            let f = parse_node(file_path, output, indent);
            make::take_anchor(cursor.mark, name, f)(token)
        }
        Ok((output, (name, false))) => make::get_anchor(cursor.mark, output, name)(token),
        Err(error) => Err((token, error)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::error::{
        Error::{self, FailedDetermineType},
    };
    use crate::data::mark::Mark;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_anchor() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        {
            let input = "@acnhor: null\nhello";
            let data_f = parse_anchor(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\nhello", Mark::new(0, 13)).into();
            let result_f = make::take_anchor::<_, Error, _, _>(
                begin_mark,
                "acnhor",
                make::null(Mark::new(0, 9), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "@: null\n\t\thello";
            let data_f = parse_anchor(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\thello", Mark::new(0, 7)).into();
            let result_f = make::take_anchor::<_, Error, _, _>(
                begin_mark,
                "",
                make::null(Mark::new(0, 3), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = ": null";
            let data_f = parse_anchor(file_path, (input, begin_mark).into(), 2);
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
