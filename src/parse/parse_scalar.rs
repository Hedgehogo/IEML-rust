use std::path::Path;

use super::{
    cursor::Cursor,
    error::{marked::MakeResult, Error::FailedDetermineType},
    parse_classic_string::parse_classic_string,
    parse_line_string::parse_line_string,
    parse_not_escaped_string::parse_not_escaped_string,
    parse_null::parse_null,
    parse_raw::parse_raw,
};
use crate::data::make;

pub(crate) fn parse_scalar<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let parsers: [fn(_, _, _, _) -> _; 3] = [
            |file_path, cursor, indent, token| {
                parse_classic_string(file_path, cursor, indent)(token)
            },
            |file_path, cursor, _indent, token| parse_line_string(file_path, cursor)(token),
            |file_path, cursor, indent, token| {
                parse_not_escaped_string(file_path, cursor, indent)(token)
            },
        ];

        let token = match parse_null(file_path, cursor)(token) {
            Ok(i) => return Ok(i),
            Err((token, _)) => token,
        };

        let mut token = token;
        for parse in parsers {
            token = match parse(file_path, cursor, indent, token) {
                Ok(i) => return Ok(i),
                Err((token, i)) => match &i.data.reason {
                    make::error::MakeErrorReason::Parse(FailedDetermineType) => token,
                    _ => return Err((token, i)),
                },
            };
        }

        parse_raw(file_path, cursor)(token)
    }
}

#[cfg(test)]
mod tests {
    use super::super::error::{
        marked::MakeError,
        Error::{self, ExpectedTab, IncompleteString},
    };
    use crate::data::mark::Mark;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_scalar() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
        {
            let input = r#"null # hello"#;
            let data_f = parse_scalar(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("# hello", Mark::new(0, 5)).into();
            let result_f = make::null::<_, Error>(begin_mark, result_output);
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = r#"hello # hello"#;
            let data_f = parse_scalar(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 13)).into();
            let result_f = make::raw::<_, Error, _>(begin_mark, result_output, "hello # hello");
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = r#"> hello # hello"#;
            let data_f = parse_scalar(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 15)).into();
            let result_f = make::string::<_, Error, _>(begin_mark, result_output, "hello # hello");
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = r#">>
		hello"#;
            let data_f = parse_scalar(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(1, 7)).into();
            let result_f = make::string::<_, Error, _>(begin_mark, result_output, "hello");
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = r#">> 	#hello
		hello
		hello
	hello"#;
            let data_f = parse_scalar(file_path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 4);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(error_mark, file_path, IncompleteString))
            );
        }
        {
            let input = r#">>
	hello"#;
            let data_f = parse_scalar(file_path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(1, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(error_mark, file_path, ExpectedTab))
            );
        }
        {
            let input = r#""hello" # hello"#;
            let data_f = parse_scalar(file_path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 15)).into();
            let result_f = make::string::<_, Error, _>(begin_mark, result_output, "hello");
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = r#""hello
	world""#;
            let data_f = parse_scalar(file_path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 6);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(error_mark, file_path, ExpectedTab))
            );
        }
        {
            let input = r#""hello"#;
            let data_f = parse_scalar(file_path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 6);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(error_mark, file_path, IncompleteString))
            );
        }
        {
            let input = r#""hello\"#;
            let data_f = parse_scalar(file_path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 7);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(MakeError::new_with(error_mark, file_path, IncompleteString))
            );
        }
    }
}
