use std::path::Path;

use super::{
    error::{
        marked::{MakeError, MakeResult, ParseResult},
        Error::FailedDetermineType,
    },
    utils::combinator::match_line,
};
use crate::data::{make, mark::Mark};
use nom::bytes::complete::tag;

pub(crate) fn parse_line_string<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    mark: Mark,
) -> ParseResult<'input, String> {
    match tag::<_, _, nom::error::Error<_>>("> ")(input) {
        Ok((input, _)) => {
            let (input, (result, mark)) = match_line(mark + Mark::new(0, 2))(input);
            Ok(((input, mark), result.into()))
        }
        Err(_) => Err(MakeError::new_with(mark, file_path, FailedDetermineType)),
    }
}

pub(crate) fn line_string<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    mark: Mark,
) -> impl FnOnce(&'input mut make::Maker) -> MakeResult<'input> {
    move |maker| {
        let map = |(output, string)| make::string(mark, output, string)(maker);
        parse_line_string(file_path, input, mark).and_then(map)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_line_string() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        assert_eq!(
            parse_line_string(file_path.as_path(), "> hello", begin_mark),
            Ok((("", Mark::new(0, 7)), "hello".into()))
        );
        assert_eq!(
            parse_line_string(file_path.as_path(), "> hello\nhello", begin_mark),
            Ok((("\nhello", Mark::new(0, 7)), "hello".into()))
        );
        assert_eq!(
            parse_line_string(file_path.as_path(), ">hello", begin_mark),
            Err(MakeError::new_with(
                begin_mark,
                file_path.clone(),
                FailedDetermineType
            ))
        );
    }
}
