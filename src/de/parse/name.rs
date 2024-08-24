use super::{
    cursor::Cursor, error::ErrorKind, read_source::ReadSource, utils::combinator::parse::match_name,
};
use crate::{
    data::name::Name,
    de::parse::{Error, LexResult},
};

pub(crate) fn name<'input, R: ReadSource + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    line_ending: bool,
) -> LexResult<'input, (Name<&'input str>, bool)> {
    let (output, (result, ending)) = match_name(cursor);

    let error_kind = if line_ending || ending {
        match Name::new(result.input) {
            Ok(i) => return Ok((output, (i, ending))),
            Err(e) => e.into(),
        }
    } else {
        ErrorKind::FailedDetermineType
    };

    Err(Error::new_with(cursor.mark, reader.path(), error_kind))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;
    use crate::data::mark::Mark;

    fn name_ref(i: &str) -> Name<&str> {
        Name::new(i.into()).unwrap()
    }

    #[test]
    fn test_name() {
        let begin_mark = Mark::new(0, 0);
        let path = "test.ieml";
        let reader = Path::new(path);
        assert_eq!(
            name(reader, ("name: value", begin_mark).into(), true),
            Ok((("value", Mark::new(0, 6)).into(), (name_ref("name"), true)))
        );
        assert_eq!(
            name(reader, ("name:\nhello", begin_mark).into(), true),
            Ok((
                ("\nhello", Mark::new(0, 5)).into(),
                (name_ref("name"), true)
            ))
        );
        assert_eq!(
            name(reader, ("name\nhello", begin_mark).into(), true),
            Ok((
                ("\nhello", Mark::new(0, 4)).into(),
                (name_ref("name"), false)
            ))
        );
        assert_eq!(
            name(reader, (": ", begin_mark).into(), true),
            Ok((("", Mark::new(0, 2)).into(), (name_ref(""), true)))
        );
        assert_eq!(
            name(reader, ("", begin_mark).into(), true),
            Ok((("", Mark::new(0, 0)).into(), (name_ref(""), false)))
        );
        assert_eq!(
            name(reader, (" name", begin_mark).into(), true),
            Err(Error::new_with(
                begin_mark,
                path,
                ErrorKind::ImpermissibleSpace
            ))
        );
        assert_eq!(
            name(reader, ("\tname", begin_mark).into(), true),
            Err(Error::new_with(
                begin_mark,
                path,
                ErrorKind::ImpermissibleTab
            ))
        );
        assert_eq!(
            name(reader, ("@name", begin_mark).into(), true),
            Err(Error::new_with(
                begin_mark,
                path,
                ErrorKind::ImpermissibleAnchor
            ))
        );
        assert_eq!(
            name(reader, ("= name", begin_mark).into(), true),
            Err(Error::new_with(
                begin_mark,
                path,
                ErrorKind::ImpermissibleTagged
            ))
        );
        assert_eq!(
            name(reader, ("name:", begin_mark).into(), true),
            Err(Error::new_with(
                begin_mark,
                path,
                ErrorKind::ImpermissibleColon
            ))
        );
        assert_eq!(
            name(reader, (" name", begin_mark).into(), false),
            Err(Error::new_with(
                begin_mark,
                path,
                ErrorKind::FailedDetermineType
            ))
        );
    }
}
