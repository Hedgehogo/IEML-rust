use crate::data::mark::Mark;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cursor<'input> {
    pub input: &'input str,
    pub mark: Mark,
}

impl<'input> Cursor<'input> {
    pub fn new(input: &'input str, mark: Mark) -> Self {
        Self { input, mark }
    }
}

impl<'input> From<Cursor<'input>> for (&'input str, Mark) {
    fn from(value: Cursor<'input>) -> Self {
        (value.input, value.mark)
    }
}

impl<'input> From<(&'input str, Mark)> for Cursor<'input> {
    fn from((input, mark): (&'input str, Mark)) -> Self {
        Cursor::new(input, mark)
    }
}