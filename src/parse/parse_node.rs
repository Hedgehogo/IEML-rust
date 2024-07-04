use std::path::Path;

use super::{cursor::Cursor, error::marked::MakeResult, parse_scalar::parse_scalar};
use crate::data::make;

pub(crate) fn parse_node<'input>(
    file_path: &'input Path,
    cursor: Cursor<'input>,
    indent: usize,
) -> impl FnOnce(make::Token) -> MakeResult<'_, 'input> {
    parse_scalar(file_path, cursor, indent)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_parse_node() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
    }
}
