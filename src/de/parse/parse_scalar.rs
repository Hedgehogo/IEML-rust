use super::{
    cursor::Cursor,
    parse_alternative::{parse_alternative, Parse},
    primitive::{
        parse_classic_string, parse_line_string, parse_not_escaped_string, parse_raw_or_null,
    },
    read_source::ReadSource,
};
use crate::{data::make, de::parse::Result};

pub(crate) fn parse_scalar<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    let parsers: [Parse<'input, R>; 4] = [
        |reader, cursor, indent, token| parse_classic_string(reader.path(), cursor, indent)(token),
        |reader, cursor, _indent, token| parse_line_string(reader.path(), cursor)(token),
        |reader, cursor, indent, token| {
            parse_not_escaped_string(reader.path(), cursor, indent)(token)
        },
        |reader, cursor, _indent, token| parse_raw_or_null(reader.path(), cursor)(token),
    ];

    parse_alternative(reader, cursor, indent, parsers.into_iter())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        data::mark::Mark,
        de::parse::{Error, ErrorKind},
    };

    use super::*;

    #[test]
    fn test_parse_scalar() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = r#"null # hello"#;
            let data_f = parse_scalar(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 12)).into();
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
