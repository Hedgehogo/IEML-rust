use std::path::Path;

use super::error::{
    marked::{MakeError, MakeResult, ParseResult},
    Error::FailedDetermineType,
};
use crate::data::{make, mark::Mark};
use nom::{
    bytes::complete::*,
    character::complete::*,
    combinator::{opt, recognize},
    sequence::tuple,
};

pub(crate) fn null<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    mark: Mark,
) -> ParseResult<'input, ()> {
    match recognize::<_, _, nom::error::Error<_>, _>(tuple((tag("null"), opt(char(' ')))))(input) {
        Ok((input, result)) => {
            let new_mark = mark + Mark::new(0, result.len());
            Ok(((input, new_mark), ()))
        }
        Err(_) => Err(MakeError::new_with(mark, file_path, FailedDetermineType)),
    }
}

pub(crate) fn parse_null<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    mark: Mark,
) -> impl FnOnce(&'input mut make::Maker) -> MakeResult<'input> {
    move |maker| {
        let map = |(output, _)| make::null(mark, output)(maker);
        null(file_path, input, mark).and_then(map)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_null() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        assert_eq!(
            null(file_path.as_path(), "null", begin_mark),
            Ok((("", Mark::new(0, 4)), ()))
        );
        assert_eq!(
            null(file_path.as_path(), "null ", begin_mark),
            Ok((("", Mark::new(0, 5)), ()))
        );
        assert_eq!(
            null(file_path.as_path(), "null# is null", begin_mark),
            Ok((("# is null", Mark::new(0, 4)), ()))
        );
        assert_eq!(
            null(file_path.as_path(), "null # is null", begin_mark),
            Ok((("# is null", Mark::new(0, 5)), ()))
        );
        assert_eq!(
            null(file_path.as_path(), " null", begin_mark),
            Err(MakeError::new_with(
                begin_mark,
                file_path.clone(),
                FailedDetermineType
            ))
        );
    }
}
