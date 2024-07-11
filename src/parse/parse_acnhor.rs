use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, MakeResult},
        Error::FailedDetermineType,
    },
    parse_node::parse_node,
    utils::combinator::{
        cursor::{anychar, char, recognize},
        many::many_till_count,
        parse::{skip_line_ending, skip_space},
    },
};
use crate::data::make;
use nom::{branch::alt, combinator::peek, sequence::tuple, Parser};

fn match_acnhor_name(input: Cursor) -> (Cursor, &str, bool) {
    let match_ending = |input| {
        let match_special = recognize(tuple((char(':'), skip_space)));
        let match_line_ending = recognize(peek(skip_line_ending));
        alt((
            match_special.map(|i| (i.input.len(), true)),
            match_line_ending.map(|i| (i.input.len(), false)),
        ))(input)
    };

    let (output, (_, special)) = many_till_count(anychar, match_ending)(input).unwrap();
    let (len_special, is_take) = special;

    let bytes = input.input.len() - output.input.len() - len_special;
    let (name, _) = input.input.split_at(bytes);

    (output, name, is_take)
}

pub(crate) fn parse_anchor<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    move |token| {
        let (output, _) = match char('@')(cursor) {
            Ok(i) => i,
            Err(_) => {
                let error = MakeError::new_with(cursor.mark, file_path, FailedDetermineType);
                return Err((token, error));
            }
        };

        let (output, name, is_take) = match_acnhor_name(output);

        if is_take {
            make::take_anchor(cursor.mark, name, parse_node(file_path, output, indent))(token)
        } else {
            make::get_anchor(cursor.mark, output, name)(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::error::{
        marked::MakeError,
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
