use std::path::Path;

use super::{error::marked::MakeResult, parse_scalar::parse_scalar};
use crate::data::{make, mark::Mark};

pub(crate) fn parse_node<'input, 'path: 'input>(
    file_path: &'path Path,
    input: &'input str,
    indent: usize,
    mark: Mark,
) -> impl FnOnce(&mut make::Maker) -> MakeResult<'input> {
    parse_scalar(file_path, input, indent, mark)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_node() {
        let begin_mark = Mark::new(0, 0);
        let file_path = PathBuf::from("test.ieml");
        let file_path = file_path.as_path();
    }
}
