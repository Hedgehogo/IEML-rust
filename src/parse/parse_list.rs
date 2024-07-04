use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult},
        Error::{ExpectedListItem, FailedDetermineType},
    },
    parse_node::parse_node,
    utils::combinator::{skip_enter, skip_indent},
};
use crate::data::{make, mark::Mark};
use nom::bytes::complete::tag;

pub(crate) fn parse_list<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let begin_mark = cursor.mark;

        let skip_special = |cursor: Cursor<'input>| {
            let match_special = tag::<_, &str, nom::error::Error<_>>("- ");
            match_special(cursor.input)
                .ok()
                .map(|(input, _)| (input, cursor.mark + Mark::new(0, 2)).into())
        };

        let skip_whitespace = |cursor: Cursor<'input>| {
            skip_enter(cursor.mark)(cursor.input)
                .ok()
                .and_then(|(input, mark)| skip_indent(indent, mark)(input).ok())
                .map(|(input, mark)| (input, mark).into())
        };

        if let Some(cursor) = skip_special(cursor) {
            return make::list(begin_mark, |token| {
                token.add(parse_node(file_path, cursor, indent))
            })(token);
        }

        let cursor = match skip_whitespace(cursor).and_then(|i| skip_special(i)) {
            Some(i) => i,
            None => {
                let error = MakeError::new_with(begin_mark, file_path, FailedDetermineType);
                return Err((token, error));
            }
        };
        make::list(begin_mark, |token| {
            let (mut token, mut cursor) = token.add(parse_node(file_path, cursor, indent))?;
            loop {
                (token, cursor) = {
                    let cursor = match skip_whitespace(cursor) {
                        Some(i) => i,
                        None => return Ok((token, cursor)),
                    };
                    let cursor = match skip_special(cursor) {
                        Some(i) => i,
                        None => {
                            let mark = cursor.mark;
                            let error = MakeError::new_with(mark, file_path, ExpectedListItem);
                            return Err((token, error));
                        }
                    };
                    token.add(parse_node(file_path, cursor, indent))?
                }
            }
        })(token)
    }
}
