use super::{
    cursor::Cursor,
    parse_alternative::{parse_alternative, Parse},
    parse_scalar::parse_scalar,
    primitive::*,
    read_file::ReadFile,
    utils::combinator::parse::{skip_blank_lines_ln, skip_indent},
};
use crate::{data::make, parse::Result};

pub(crate) fn parse_node_on_own_line<'input, R: ReadFile + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    let parsers: [Parse<'input, R>; 7] = [
        |reader, cursor, indent, token| parse_tagged(reader, cursor, indent)(token),
        |reader, cursor, indent, token| parse_anchor(reader, cursor, indent)(token),
        |reader, cursor, indent, token| parse_file(reader, cursor, indent)(token),
        |reader, cursor, _indent, token| parse_short_list(reader, cursor)(token),
        |reader, cursor, indent, token| parse_list(reader, cursor, indent)(token),
        |reader, cursor, indent, token| parse_map(reader, cursor, indent)(token),
        |reader, cursor, indent, token| parse_scalar(reader.path(), cursor, indent)(token),
    ];

    parse_alternative(reader, cursor, indent, parsers.into_iter())
}

pub(crate) fn parse_node<'input, R: ReadFile + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| {
        let skip_whitespace = |cursor| {
            skip_blank_lines_ln(cursor).and_then(|(cursor, _)| skip_indent(indent)(cursor))
        };

        let line_parsers: [Parse<'input, R>; 7] = [
            |reader, cursor, indent, token| parse_tagged(reader, cursor, indent)(token),
            |reader, cursor, indent, token| parse_anchor(reader, cursor, indent)(token),
            |reader, cursor, indent, token| parse_file(reader, cursor, indent)(token),
            |reader, cursor, _indent, token| parse_short_list(reader, cursor)(token),
            |reader, cursor, indent, token| parse_list_one(reader, cursor, indent)(token),
            |reader, cursor, indent, token| parse_map_one(reader, cursor, indent)(token),
            |reader, cursor, indent, token| parse_scalar(reader.path(), cursor, indent)(token),
        ];

        match skip_whitespace(cursor) {
            Ok((cursor, _)) => parse_node_on_own_line(reader, cursor, indent)(token),
            Err(_) => parse_alternative(reader, cursor, indent, line_parsers.into_iter())(token),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::mark::Mark,
        parse::{test_utils::*, Error, ErrorKind},
    };
    use std::path::Path;

    use super::*;

    #[test]
    fn test_parse_node() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = "null # hello";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 12)).into();
            let result_f = make::null::<_, ErrorKind>(begin_mark, result_output);
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "- null # hello\n\t\t- null";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\t- null", Mark::new(0, 14)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                token.add(make::null(Mark::new(0, 2), result_output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "[null, null]";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 12)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                let (token, output) = token.add(make::null(Mark::new(0, 1), result_output))?;
                let (token, output) = token.add(make::null(Mark::new(0, 7), output))?;
                Ok((token, output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "first: null # hello\n\t\tsecond: null";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\n\t\tsecond: null", Mark::new(0, 19)).into();
            let result_f = make::map::<_, ErrorKind, _>(begin_mark, |token| {
                token.add(
                    Mark::new(0, 0),
                    name("first"),
                    make::null(Mark::new(0, 7), result_output),
                )
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "= tag: null";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
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
            let files = Files::from([
                (Path::new("test.ieml"), "< subtest\n\t\tanchor: null".into()),
                (Path::new("subtest"), "null".into()),
            ]);
            let reader = Reader::new(&files, path);
            let input = files.get(reader.path()).unwrap().as_str();
            let data_f = parse_node(&reader, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(1, 14)).into();
            let result_f = make::file::<_, ErrorKind, _, _>(
                begin_mark,
                Path::new("subtest").into(),
                |token| {
                    let mark = Mark::new(1, 10);
                    token.add(mark, name("anchor"), make::null(mark, result_output))
                },
                make::null(begin_mark, ("", Mark::new(0, 4)).into()),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "@anchor: null\nhello";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\nhello", Mark::new(0, 13)).into();
            let result_f = make::take_anchor::<_, ErrorKind, _, _>(
                begin_mark,
                name("anchor"),
                make::null(Mark::new(0, 9), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = "\n\t\tnull # hello";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(1, 14)).into();
            let result_f = make::null::<_, ErrorKind>(Mark::new(1, 2), result_output);
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "\n\t\t- null # hello\n\t\t- null";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(2, 8)).into();
            let result_f = make::list::<_, ErrorKind, _>(Mark::new(1, 2), |token| {
                let (token, output) = token.add(make::null(Mark::new(1, 4), result_output))?;
                let (token, output) = token.add(make::null(Mark::new(2, 4), output))?;
                Ok((token, output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "\n\t\t[null, null]";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(1, 14)).into();
            let result_f = make::list::<_, ErrorKind, _>(Mark::new(1, 2), |token| {
                let (token, output) = token.add(make::null(Mark::new(1, 3), result_output))?;
                let (token, output) = token.add(make::null(Mark::new(1, 9), output))?;
                Ok((token, output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "\n\t\tfirst: null # hello\n\t\tsecond: null";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(2, 14)).into();
            let result_f = make::map::<_, ErrorKind, _>(Mark::new(1, 2), |token| {
                let (token, output) = token.add(
                    Mark::new(1, 2),
                    name("first"),
                    make::null(Mark::new(1, 9), result_output),
                )?;
                let (token, output) = token.add(
                    Mark::new(2, 2),
                    name("second"),
                    make::null(Mark::new(2, 10), output),
                )?;
                Ok((token, output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "\n\t\t= tag: null";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(1, 13)).into();
            let result_f = make::tagged::<_, ErrorKind, _, _>(
                Mark::new(1, 2),
                name("tag"),
                make::null(Mark::new(1, 9), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let files = Files::from([
                (
                    Path::new("test.ieml"),
                    "\n\t\t< subtest\n\t\tanchor: null".into(),
                ),
                (Path::new("subtest"), "null".into()),
            ]);
            let reader = Reader::new(&files, path);
            let input = files.get(reader.path()).unwrap().as_str();
            let data_f = parse_node(&reader, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(2, 14)).into();
            let result_f = make::file::<_, ErrorKind, _, _>(
                Mark::new(1, 2),
                Path::new("subtest").into(),
                |token| {
                    let mark = Mark::new(2, 10);
                    token.add(mark, name("anchor"), make::null(mark, result_output))
                },
                make::null(begin_mark, ("", Mark::new(0, 4)).into()),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "\n\t\t@anchor: null\nhello";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("\nhello", Mark::new(1, 15)).into();
            let result_f = make::take_anchor::<_, ErrorKind, _, _>(
                Mark::new(1, 2),
                name("anchor"),
                make::null(Mark::new(1, 11), result_output),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "<hello";
            let data_f = parse_node(path, (input, begin_mark).into(), 2);
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
