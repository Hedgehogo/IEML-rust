use super::{
    cursor::Cursor, error::marked::ParseResult, parse_scalar::parse_scalar, read_file::ReadFile,
};
use crate::data::make;

pub(crate) fn parse_node<'input, R: ReadFile + ?Sized>(
    reader: &'input R,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> ParseResult<'_, 'input> {
    parse_scalar(reader.path(), cursor, indent)
}

#[cfg(test)]
mod tests {
    use crate::data::mark::Mark;
    use std::path::Path;

    use super::*;

    #[test]
    fn test_parse_node() {
        let begin_mark = Mark::new(0, 0);
        let path = Path::new("test.ieml");
    }
}
