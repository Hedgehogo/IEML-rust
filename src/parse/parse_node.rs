use super::{
    cursor::Cursor, error::marked::MakeResult, parse_scalar::parse_scalar, read_file::ReadFile,
};
use crate::data::make;

pub(crate) fn parse_node<'input, R: ReadFile<'input>>(
    reader: R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    parse_scalar(reader.path(), cursor, indent)
}

#[cfg(test)]
mod tests {
    use crate::data::mark::Mark;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_node() {
        let begin_mark = Mark::new(0, 0);
        let path = PathBuf::from("test.ieml");
        let path = path.as_path();
    }
}
