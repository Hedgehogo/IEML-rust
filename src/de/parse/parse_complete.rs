use super::{
    cursor::Cursor,
    parse_node::parse_node_on_own_line,
    read_source::ReadSource,
    utils::combinator::parse::{skip_blank_line, skip_blank_lines_ln},
};
use crate::{
    data::make,
    de::parse::{Error, ErrorKind, RateError, Result},
};

pub(crate) fn parse_complete<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
) -> impl FnOnce(make::Token) -> Result<'_, 'input> {
    move |token| {
        let (cursor, _) = skip_blank_lines_ln(cursor).unwrap_or((cursor, 0));
        let (token, cursor) = parse_node_on_own_line(reader, cursor, 0)(token)?;
        let (cursor, _) = skip_blank_lines_ln(cursor).unwrap_or((cursor, 0));
        let cursor = skip_blank_line(cursor);

        if cursor.input.is_empty() {
            Ok((token, cursor))
        } else {
            let error_kind = ErrorKind::IncompleteDocument;
            let error = Error::new_with(cursor.mark, reader.path(), error_kind);
            Err(RateError::Unrecoverable((token.error(), error)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        data::mark::Mark,
        de::parse::{Error, ErrorKind},
    };
    use std::path::Path;

    #[test]
    fn test_parse_complete() {
        let begin_mark = Mark::new(0, 0);
        let path = "test.ieml";
        let reader = Path::new(path);
        {
            let input = "null # hello";
            let data_f = parse_complete(reader, (input, begin_mark).into());
            let data = make::make(begin_mark, data_f).unwrap();
            let result_output = ("", Mark::new(0, 12)).into();
            let result_f = make::null::<_, ErrorKind>(begin_mark, result_output);
            let result = make::make(begin_mark, result_f).unwrap();
            assert_eq!(data, result);
        }
        {
            let input = "null # hello\nnull";
            let data_f = parse_complete(reader, (input, begin_mark).into());
            let error_mark = Mark::new(1, 0);
            assert_eq!(
                make::make(begin_mark, data_f),
                Err(Error::new_with(
                    error_mark,
                    path,
                    ErrorKind::IncompleteDocument
                ))
            );
        }
    }
}
