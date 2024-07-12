use std::path::Path;

use super::{
    cursor::Cursor,
    error::{
        marked::{MakeError, ParseResult},
        Error::{FailedDetermineType, ImpermissibleSpace, ImpermissibleTab},
    },
    utils::combinator::parse::match_name,
};

pub(crate) fn name<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
    line_ending: bool,
) -> ParseResult<'input, (&'input str, bool)> {
    let (output, (result, ending)) = match_name(cursor);

    let error_reason = if line_ending || ending {
        match result.input.chars().next() {
            Some(' ') => ImpermissibleSpace,
            Some('\t') => ImpermissibleTab,
            _ => return Ok((output, (result.input, ending))),
        }
    } else {
        FailedDetermineType
    };

    Err(MakeError::new_with(cursor.mark, path, error_reason))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_name() {
        let begin_mark = Mark::new(0, 0);
        let path = PathBuf::from("test.ieml");
        let path = path.as_path();
        assert_eq!(
            name(path, ("name: value", begin_mark).into(), true),
            Ok((("value", Mark::new(0, 6)).into(), ("name", true)))
        );
        assert_eq!(
            name(path, ("name:\nhello", begin_mark).into(), true),
            Ok((("\nhello", Mark::new(0, 5)).into(), ("name", true)))
        );
        assert_eq!(
            name(path, ("name\nhello", begin_mark).into(), true),
            Ok((("\nhello", Mark::new(0, 4)).into(), ("name", false)))
        );
        assert_eq!(
            name(path, (": ", begin_mark).into(), true),
            Ok((("", Mark::new(0, 2)).into(), ("", true)))
        );
        assert_eq!(
            name(path, ("", begin_mark).into(), true),
            Ok((("", Mark::new(0, 0)).into(), ("", false)))
        );
        assert_eq!(
            name(path, (" name", begin_mark).into(), true),
            Err(MakeError::new_with(
                begin_mark,
                path,
                ImpermissibleSpace
            ))
        );
        assert_eq!(
            name(path, ("\tname", begin_mark).into(), true),
            Err(MakeError::new_with(begin_mark, path, ImpermissibleTab))
        );
        assert_eq!(
            name(path, (" name", begin_mark).into(), false),
            Err(MakeError::new_with(
                begin_mark,
                path,
                FailedDetermineType
            ))
        );
    }
}
