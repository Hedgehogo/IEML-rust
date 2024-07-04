use std::path::Path;

use super::{
    error::{
        marked::{MakeError, MakeResult},
        Error::{ExpectedListItem, FailedDetermineType},
    },
    parse_node::parse_node,
    utils::combinator::{skip_enter, skip_indent},
};
use crate::data::{make, mark::Mark};
use nom::bytes::complete::tag;

pub(crate) fn parse_list<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    indent: usize,
    mark: Mark,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let begin_mark = mark;

        let skip_special = |input, mark| {
            let match_special = tag::<_, _, nom::error::Error<_>>("- ");
            match_special(input)
                .ok()
                .map(|(input, _)| (input, mark + Mark::new(0, 2)))
        };

        let skip_whitespace = |input, mark| {
            skip_enter(mark)(input)
                .ok()
                .and_then(|(input, mark)| skip_indent(indent, mark)(input).ok())
        };

        if let Some((input, mark)) = skip_special(input, mark) {
            return make::list(begin_mark, |token| {
                token.add(parse_node(file_path, input, indent, mark))
            })(token);
        }

        let (input, mark) = match skip_whitespace(input, mark)
            .and_then(|(input, mark)| skip_special(input, mark))
        {
            Some(i) => i,
            None => {
                let error = MakeError::new_with(begin_mark, file_path, FailedDetermineType);
                return Err((token, error));
            }
        };
        make::list(begin_mark, |token| {
            let (mut token, (mut input, mut mark)) =
                token.add(parse_node(file_path, input, indent, mark))?;
            loop {
                (token, (input, mark)) = {
                    let (input, mark) = match skip_whitespace(input, mark) {
                        Some(i) => i,
                        None => return Ok((token, (input, mark))),
                    };
                    let (input, mark) = match skip_special(input, mark) {
                        Some(i) => i,
                        None => {
                            let error = MakeError::new_with(mark, file_path, ExpectedListItem);
                            return Err((token, error));
                        }
                    };
                    token.add(parse_node(file_path, input, indent, mark))?
                }
            }
        })(token)
    }
}
