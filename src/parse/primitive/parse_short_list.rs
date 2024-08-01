use std::path::Path;

use super::super::{
    cursor::Cursor,
    read_source::ReadSource,
    utils::combinator::cursor::{anychar, char, none_of, one_of, recognize},
};
use super::parse_classic_string;
use crate::{
    data::{make, name::Name},
    parse::{
        utils::combinator::parse::match_line, Error, ErrorKind, LexResult, ListResult, RateError,
        Result,
    },
};
use nom::{
    branch::alt,
    combinator::{not, value},
    multi::many1_count,
    sequence::tuple,
    IResult, Parser,
};

fn lex_beginning<'input>(path: &'input Path, cursor: Cursor<'input>) -> LexResult<'input, ()> {
    match char('[')(cursor) {
        Ok((cursor, _)) => Ok((cursor, ())),

        Err(_) => {
            let kind = ErrorKind::FailedDetermineType;
            Err(Error::new_with(cursor.mark, path, kind))
        }
    }
}

fn lex_special<'input>(path: &'input Path, cursor: Cursor<'input>) -> LexResult<'input, bool> {
    let ending = char(']');
    let separator = tuple((char(','), char(' ')));

    match alt((value(false, ending), value(true, separator)))(cursor) {
        Ok((cursor, value)) => Ok((cursor, value)),

        Err(_) => {
            let kind = ErrorKind::IncompleteShortList;
            Err(Error::new_with(cursor.mark, path, kind))
        }
    }
}

fn not_any_ending<'input>(cursor: Cursor<'input>) -> IResult<Cursor<'input>, ()> {
    let ending = char(']').map(|_| ());
    let separator = tuple((char(','), char(' '))).map(|_| ());
    let comment = tuple((char('#'), one_of("! "))).map(|_| ());
    not(alt((ending, separator, comment)))(cursor)
}

fn lex_raw_or_null<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> LexResult<'input, &'input str> {
    let lexer = many1_count(tuple((not_any_ending, none_of("\"\n<>"))));

    match recognize(lexer)(cursor) {
        Ok((cursor, result)) => Ok((cursor, result.input)),

        Err(_) => {
            let kind = ErrorKind::FailedDetermineType;
            Err(Error::new_with(cursor.mark, path, kind))
        }
    }
}

fn parse_raw_or_null<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| match lex_raw_or_null(path, cursor) {
        Ok((output, "null")) => make::null(cursor.mark, output)(token),
        Ok((output, result)) => make::raw(cursor.mark, output, result)(token),
        Err(error) => Err(RateError::Recoverable((token, error))),
    }
}

fn lex_anchor_request<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> LexResult<'input, Name<&'input str>> {
    let name = many1_count(tuple((not_any_ending, anychar)));
    let error_kind = match tuple((char('@'), recognize(name)))(cursor) {
        Ok((cursor, (_, result))) => match Name::new(result.input) {
            Ok(result) => return Ok((cursor, result)),
            Err(error) => error.into(),
        },

        Err(_) => ErrorKind::FailedDetermineType,
    };

    Err(Error::new_with(cursor.mark, path, error_kind))
}

fn parse_anchor_request<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| match lex_anchor_request(path, cursor) {
        Ok((output, name)) => make::anchor_request(cursor.mark, output, name)(token),

        Err(error) => match error.data.kind {
            make::ErrorKind::Parse(ErrorKind::FailedDetermineType) => {
                Err(RateError::Recoverable((token, error)))
            }

            _ => Err(RateError::Unrecoverable((token.error(), error))),
        },
    }
}

fn parse_short_list_item<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::ListToken) -> ListResult<'_, 'input> {
    move |token| {
        type Closure<'input, 'maker, R> =
            fn(&'input R, Cursor<'input>, make::ListToken<'maker>) -> ListResult<'maker, 'input>;
        let parsers: [Closure<'_, '_, R>; 4] = [
            |reader, cursor, token| token.add(parse_anchor_request(reader.path(), cursor)),
            |reader, cursor, token| token.add(parse_short_list(reader, cursor)),
            |reader, cursor, token| token.add(parse_classic_string(reader.path(), cursor, 0)),
            |reader, cursor, token| token.add(parse_raw_or_null(reader.path(), cursor)),
        ];

        let mut token = token;
        for parse in parsers {
            token = match parse(reader, cursor, token) {
                Ok(i) => return Ok(i),

                Err(error) => match error {
                    RateError::Recoverable((token, _)) => token,

                    RateError::Unrecoverable(error) => {
                        return Err(RateError::Unrecoverable(error));
                    }
                },
            };
        }

        let error_kind = ErrorKind::FailedDetermineType;
        let error = Error::new_with(cursor.mark, reader.path(), error_kind);
        Err(RateError::Unrecoverable((token.error(), error)))
    }
}

pub(crate) fn parse_short_list<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| {
        let item_cursor = match lex_beginning(reader.path(), cursor) {
            Ok((item_cursor, _)) => item_cursor,
            Err(error) => return Err(RateError::Recoverable((token, error))),
        };

        make::list(cursor.mark, |token| {
            if let Ok((output, false)) = lex_special(reader.path(), item_cursor) {
                return Ok((token, output));
            }

            let (_, line_cursor) = match_line(item_cursor);

            let (mut token, mut cursor) = (token, line_cursor);
            let (token, cursor) = loop {
                (token, cursor) = {
                    let (token, cursor) = parse_short_list_item(reader, cursor)(token)?;
                    match lex_special(reader.path(), cursor) {
                        Ok((cursor, true)) => (token, cursor),
                        Ok((cursor, false)) => break (token, cursor),
                        Err(error) => return Err(RateError::Unrecoverable((token.error(), error))),
                    }
                }
            };

            let consumed_len = line_cursor.input.len() - cursor.input.len();
            let (_, output) = item_cursor.input.split_at(consumed_len);
            let cursor = (output, cursor.mark).into();
            Ok((token, cursor))
        })(token)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_parse_short_list() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        {
            let input = "[]";
            let data_f = parse_short_list(path, (input, begin_mark).into());
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 2)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                return Ok((token, result_output));
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "[null]";
            let data_f = parse_short_list(path, (input, begin_mark).into());
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 6)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                token.add(make::null(Mark::new(0, 1), result_output))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "[null, hello]";
            let data_f = parse_short_list(path, (input, begin_mark).into());
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(0, 13)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                let (token, cursor) = token.add(make::null(Mark::new(0, 1), result_cursor))?;
                let (token, cursor) = token.add(make::raw(Mark::new(0, 7), cursor, "hello"))?;
                Ok((token, cursor))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "[null , hello]";
            let data_f = parse_short_list(path, (input, begin_mark).into());
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(0, 14)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                let (token, cursor) =
                    token.add(make::raw(Mark::new(0, 1), result_cursor, "null "))?;
                let (token, cursor) = token.add(make::raw(Mark::new(0, 8), cursor, "hello"))?;
                Ok((token, cursor))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "[\"hello\", null]";
            let data_f = parse_short_list(path, (input, begin_mark).into());
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(0, 15)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                let (token, cursor) =
                    token.add(make::string(Mark::new(0, 1), result_cursor, "hello"))?;
                let (token, cursor) = token.add(make::null(Mark::new(0, 10), cursor))?;
                Ok((token, cursor))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "[[null, null], null]";
            let data_f = parse_short_list(path, (input, begin_mark).into());
            let data = make::make(begin_mark, data_f).unwrap();
            let result_cursor = ("", Mark::new(0, 20)).into();
            let result_f = make::list::<_, ErrorKind, _>(begin_mark, |token| {
                let (token, cursor) = token.add(make::list(Mark::new(0, 1), |token| {
                    let (token, cursor) = token.add(make::null(Mark::new(0, 2), result_cursor))?;
                    let (token, cursor) = token.add(make::null(Mark::new(0, 8), cursor))?;
                    Ok((token, cursor))
                }))?;
                let (token, cursor) = token.add(make::null(Mark::new(0, 15), cursor))?;
                Ok((token, cursor))
            });
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "[\"\n\", null]";
            let data_f = parse_short_list(path, (input, begin_mark).into());
            let error_mark = Mark::new(0, 2);
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
            let input = "[null# , null]";
            let data_f = parse_short_list(path, (input, begin_mark).into());
            let error_mark = Mark::new(0, 5);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::IncompleteShortList
                ))
            );
        }
        {
            let input = "[, null]";
            let data_f = parse_short_list(path, (input, begin_mark).into());
            let error_mark = Mark::new(0, 1);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::FailedDetermineType
                ))
            );
        }
        {
            let input = "]null]";
            let data_f = parse_short_list(path, (input, begin_mark).into());
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
