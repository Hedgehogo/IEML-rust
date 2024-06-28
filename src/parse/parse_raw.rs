use std::path::Path;

use super::error::{
    marked::{MakeError, MakeResult, ParseResult},
    Error::FailedDetermineType,
};
use crate::data::{make, mark::Mark};
use nom::multi::many1_count;
use nom::{character::complete::*, combinator::recognize};

pub(crate) fn parse_raw<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    mark: Mark,
) -> ParseResult<'input, String> {
    match recognize(many1_count(none_of::<_, _, nom::error::Error<_>>("\"\n<>")))(input) {
        Ok((input, result)) => {
            let new_mark = mark + Mark::new(0, result.len());
            Ok(((input, new_mark), result.into()))
        }
        Err(_) => Err(MakeError::new_with(mark, file_path, FailedDetermineType)),
    }
}

pub(crate) fn raw<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    mark: Mark,
) -> impl FnOnce(&'input mut make::Maker) -> MakeResult<'input> {
    move |maker| {
        let map = |(output, raw)| make::raw(mark, output, raw)(maker);
        parse_raw(file_path, input, mark).and_then(map)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_raw() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        assert_eq!(
            parse_raw(file_path.as_path(), "hello", begin_mark),
            Ok((("", Mark::new(0, 5)), "hello".into()))
        );
        assert_eq!(
            parse_raw(file_path.as_path(), "hello\n", begin_mark),
            Ok((("\n", Mark::new(0, 5)), "hello".into()))
        );
        assert_eq!(
            parse_raw(file_path.as_path(), "< \n", begin_mark),
            Err(MakeError::new_with(
                begin_mark,
                file_path.clone(),
                FailedDetermineType
            ))
        );
    }
}
