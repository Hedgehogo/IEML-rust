use std::path::Path;

use super::{
    cursor::Cursor, parse_classic_string::parse_classic_string,
    parse_line_string::parse_line_string, parse_not_escaped_string::parse_not_escaped_string,
    parse_null::parse_null, parse_raw::parse_raw,
};
use super::{ErrorKind, RateError, Result};
use crate::data::make;

pub(crate) fn parse_scalar<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| {
        let parsers: [fn(_, _, _, _) -> _; 4] = [
            |path, cursor, _indent, token| parse_null(path, cursor)(token),
            |path, cursor, indent, token| parse_classic_string(path, cursor, indent)(token),
            |path, cursor, _indent, token| parse_line_string(path, cursor)(token),
            |path, cursor, indent, token| parse_not_escaped_string(path, cursor, indent)(token),
        ];

        let mut token = token;
        for parse in parsers {
            token = match parse(path, cursor, indent, token) {
                Ok(i) => return Ok(i),

                Err(error) => match error {
                    RateError::Recoverable((token, _)) => token,

                    RateError::Unrecoverable(error) => {
                        return Err(RateError::Unrecoverable(error));
                    }
                },
            };
        }

        parse_raw(path, cursor)(token)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Error;
    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_parse_scalar() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = r#"null # hello"#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("# hello", Mark::new(0, 5)).into();
            let result_f = make::null::<_, ErrorKind>(begin_mark, result_output);
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = r#"hello # hello"#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 13)).into();
            let result_f = make::raw::<_, ErrorKind, _>(begin_mark, result_output, "hello # hello");
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = r#"> hello # hello"#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 15)).into();
            let result_f =
                make::string::<_, ErrorKind, _>(begin_mark, result_output, "hello # hello");
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = r#">>
		hello"#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(1, 7)).into();
            let result_f = make::string::<_, ErrorKind, _>(begin_mark, result_output, "hello");
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = r#">> 	#hello
		hello
		hello
	hello"#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 4);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::IncompleteString
                ))
            );
        }
        {
            let input = r#">>
	hello"#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(1, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(error_mark, path, ErrorKind::ExpectedTab))
            );
        }
        {
            let input = r#""hello" # hello"#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 15)).into();
            let result_f = make::string::<_, ErrorKind, _>(begin_mark, result_output, "hello");
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = r#""hello
	world""#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(1, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(error_mark, path, ErrorKind::ExpectedTab))
            );
        }
        {
            let input = r#""hello"#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 6);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::IncompleteString
                ))
            );
        }
        {
            let input = r#""hello\"#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let error_mark = Mark::new(0, 7);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::IncompleteString
                ))
            );
        }
    }
}
