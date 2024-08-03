use std::path::Path;

use super::super::{
    cursor::Cursor,
    parse_complete::parse_complete,
    read_source::ReadSource,
    utils::combinator::{
        cursor::char,
        parse::{match_line, skip_blank_lines_ln, skip_indent},
    },
};
use super::parse_map::parse_map_item;
use crate::{
    data::make,
    de::parse::{Error, ErrorKind, LexResult, MapResult, RateError, Result},
};
use nom::sequence::tuple;

fn lex_path<'input>(path: &'input Path, cursor: Cursor<'input>) -> LexResult<'input, &'input Path> {
    match tuple((char('<'), char(' ')))(cursor) {
        Ok((cursor, _)) => {
            let (cursor, result) = match_line(cursor);
            return Ok((cursor, Path::new(result.input)));
        }
        Err(_) => {
            let error_kind = ErrorKind::FailedDetermineType;
            Err(Error::new_with(cursor.mark, path, error_kind))
        }
    }
}

fn parse_anchors<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::MapToken) -> MapResult<'_, 'input> {
    move |token| {
        let skip_whitespace = |cursor| {
            let (cursor, _) = skip_blank_lines_ln(cursor).ok()?;
            let (cursor, _) = skip_indent(indent)(cursor).ok()?;
            Some(cursor)
        };

        let (mut cursor, mut token) = (cursor, token);
        loop {
            (token, cursor) = match skip_whitespace(cursor) {
                Some(cursor) => {
                    let error_kind = ErrorKind::ExpectedMapKey;
                    parse_map_item(reader, cursor, indent, error_kind)(token).map_err(|error| {
                        match error {
                            RateError::Recoverable((token, error)) => {
                                RateError::Unrecoverable((token.error(), error))
                            }
                            RateError::Unrecoverable(error) => RateError::Unrecoverable(error),
                        }
                    })?
                }
                None => return Ok((token, cursor)),
            }
        }
    }
}

fn parse_child<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    path: &'input Path,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| {
        let f = move |token, reader: &R::Child, inner_cursor: Cursor<'_>| {
            let (token, _) = parse_complete(reader, inner_cursor)(token)?;
            Ok((token, cursor))
        };
        match reader.read_child(token, path, f) {
            Ok(i) => i,
            Err(token) => {
                let error_kind = ErrorKind::NonexistentDocument;
                let error = Error::new_with(cursor.mark, reader.path(), error_kind);
                Err(RateError::Unrecoverable((token.error(), error)))
            }
        }
    }
}

pub(crate) fn parse_document<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| match lex_path(reader.path(), cursor) {
        Ok((new_cursor, path)) => {
            let anchors = parse_anchors(reader, new_cursor, indent);

            let f = parse_child(reader, cursor, path);

            make::document(cursor.mark, path.into(), anchors, f)(token)
        }
        Err(error) => Err(RateError::Recoverable((token, error))),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::{path::Path, result};

    use crate::{
        data::{data::Data, make, mark::Mark},
        de::parse::test_utils::*,
    };

    use super::*;

    fn parse<'input>(
        begin_mark: Mark,
        reader: &'input Reader,
        documents: &'input Documents,
    ) -> result::Result<(Data, Cursor<'input>), Error> {
        let cursor = (documents.get(reader.path()).unwrap().as_str(), begin_mark).into();
        let data_f = parse_document(reader, cursor, 2);
        make::make(begin_mark, data_f)
    }

    #[test]
    fn test_parse_document() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test");
        {
            let documents = Documents::from([
                (Path::new("test"), "< subtest".into()),
                (Path::new("subtest"), "null".into()),
            ]);
            let reader = Reader::new(&documents, path);
            let data = parse(begin_mark, &reader, &documents).unwrap();
            let result_output = ("", Mark::new(0, 9)).into();
            let result_f = make::document::<_, ErrorKind, _, _>(
                begin_mark,
                Path::new("subtest").into(),
                |token| Ok((token, result_output)),
                make::null(begin_mark, ("", Mark::new(0, 4)).into()),
            );
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let documents = Documents::from([
                (Path::new("test"), "< subtest\n\t\tanchor: null".into()),
                (Path::new("subtest"), "null".into()),
            ]);
            let reader = Reader::new(&documents, path);
            let data = parse(begin_mark, &reader, &documents).unwrap();
            let result_output = ("", Mark::new(1, 14)).into();
            let result_f = make::document::<_, ErrorKind, _, _>(
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
            let documents = Documents::from([
                (
                    Path::new("test"),
                    "< subtest\n# hello\n\t\tanchor: null".into(),
                ),
                (Path::new("subtest"), "null".into()),
            ]);
            let reader = Reader::new(&documents, path);
            let data = parse(begin_mark, &reader, &documents).unwrap();
            let result_output = ("", Mark::new(2, 14)).into();
            let result_f = make::document::<_, ErrorKind, _, _>(
                begin_mark,
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
            let documents = Documents::from([
                (Path::new("test"), "< nonexistent".into()),
                (Path::new("subtest"), "null".into()),
            ]);
            let reader = Reader::new(&documents, path);
            let error_mark = Mark::new(0, 0);
            assert_eq!(
                parse(begin_mark, &reader, &documents),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::NonexistentDocument
                ))
            );
        }
        {
            let documents = Documents::from([
                (Path::new("test"), "< subtest\n\t\tanchor".into()),
                (Path::new("subtest"), "null".into()),
            ]);
            let reader = Reader::new(&documents, path);
            let error_mark = Mark::new(1, 2);
            assert_eq!(
                parse(begin_mark, &reader, &documents),
                Err(Error::new_with(error_mark, path, ErrorKind::ExpectedMapKey))
            );
        }
        {
            let documents = Documents::from([
                (Path::new("test"), "< subtest".into()),
                (Path::new("subtest"), "null\nhello".into()),
            ]);
            let reader = Reader::new(&documents, path);
            let error_mark = Mark::new(1, 0);
            assert_eq!(
                parse(begin_mark, &reader, &documents),
                Err(Error::new_with(
                    error_mark,
                    Path::new("subtest"),
                    ErrorKind::IncompleteDocument
                ))
            );
        }
    }
}
