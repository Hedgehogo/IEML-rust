use std::path::Path;

use crate::{
    data::name::NameRef,
    parse::{Error, LexResult},
};

use super::{
    cursor::Cursor, error::ErrorKind::FailedDetermineType, utils::combinator::parse::match_name,
};

pub(crate) fn name<'input>(
    path: &'input Path,
    cursor: Cursor<'input>,
    line_ending: bool,
) -> LexResult<'input, (NameRef<'input>, bool)> {
    let (output, (result, ending)) = match_name(cursor);

    let error_kind = if line_ending || ending {
        match NameRef::new(result.input) {
            Ok(i) => return Ok((output, (i, ending))),
            Err(e) => e.into(),
        }
    } else {
        FailedDetermineType
    };

    Err(Error::new_with(cursor.mark, path, error_kind))
}

#[cfg(test)]
mod tests {
    use crate::data::mark::Mark;

    use super::super::error::ErrorKind::{ImpermissibleSpace, ImpermissibleTab};

    use super::*;

    fn name_ref(i: &str) -> NameRef {
        NameRef::new(i.into()).unwrap()
    }

    #[test]
    fn test_name() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
        assert_eq!(
            name(path, ("name: value", begin_mark).into(), true),
            Ok((("value", Mark::new(0, 6)).into(), (name_ref("name"), true)))
        );
        assert_eq!(
            name(path, ("name:\nhello", begin_mark).into(), true),
            Ok((
                ("\nhello", Mark::new(0, 5)).into(),
                (name_ref("name"), true)
            ))
        );
        assert_eq!(
            name(path, ("name\nhello", begin_mark).into(), true),
            Ok((
                ("\nhello", Mark::new(0, 4)).into(),
                (name_ref("name"), false)
            ))
        );
        assert_eq!(
            name(path, (": ", begin_mark).into(), true),
            Ok((("", Mark::new(0, 2)).into(), (name_ref(""), true)))
        );
        assert_eq!(
            name(path, ("", begin_mark).into(), true),
            Ok((("", Mark::new(0, 0)).into(), (name_ref(""), false)))
        );
        assert_eq!(
            name(path, (" name", begin_mark).into(), true),
            Err(Error::new_with(begin_mark, path, ImpermissibleSpace))
        );
        assert_eq!(
            name(path, ("\tname", begin_mark).into(), true),
            Err(Error::new_with(begin_mark, path, ImpermissibleTab))
        );
        assert_eq!(
            name(path, (" name", begin_mark).into(), false),
            Err(Error::new_with(begin_mark, path, FailedDetermineType))
        );
    }
}
