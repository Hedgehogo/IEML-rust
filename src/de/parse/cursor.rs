use std::str::{CharIndices, Chars};

use crate::data::mark::Mark;
use nom::{InputIter, InputLength};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Cursor<'input> {
    pub input: &'input str,
    pub mark: Mark,
}

impl<'input> Cursor<'input> {
    pub fn new(input: &'input str, mark: Mark) -> Self {
        Self { input, mark }
    }

    pub fn chars(self) -> Iter<'input> {
        Iter {
            iter: self.input.chars(),
            mark: self.mark,
        }
    }

    pub fn char_indices(self) -> IterIndices<'input> {
        IterIndices {
            iter: self.input.char_indices(),
            mark: self.mark,
        }
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

impl<'input> IntoIterator for Cursor<'input> {
    type Item = char;

    type IntoIter = Iter<'input>;

    fn into_iter(self) -> Self::IntoIter {
        self.chars()
    }
}

impl<'input> InputLength for Cursor<'input> {
    fn input_len(&self) -> usize {
        self.input.len()
    }
}

impl<'input> InputIter for Cursor<'input> {
    type Item = char;

    type Iter = IterIndices<'input>;

    type IterElem = Iter<'input>;

    fn iter_indices(&self) -> Self::Iter {
        self.char_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.chars()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        for (i, ch) in self.char_indices() {
            if predicate(ch) {
                return Some(i);
            }
        }
        None
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        let mut cnt = 0;
        for (index, _) in self.char_indices() {
            if cnt == count {
                return Ok(index);
            }
            cnt += 1;
        }
        if cnt == count {
            return Ok(self.input.len());
        }
        Err(nom::Needed::Unknown)
    }
}

pub struct Iter<'input> {
    iter: Chars<'input>,
    mark: Mark,
}

impl<'input> Iter<'input> {
    pub fn cursor(self) -> Cursor<'input> {
        Cursor::new(self.iter.as_str(), self.mark)
    }
}

impl<'input> Iterator for Iter<'input> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(ch) => {
                match ch {
                    '\n' => self.mark = self.mark.newline(),
                    _ => self.mark = self.mark + Mark::new(0, 1),
                }
                Some(ch)
            }
            None => None,
        }
    }
}

pub struct IterIndices<'input> {
    iter: CharIndices<'input>,
    mark: Mark,
}

impl<'input> IterIndices<'input> {
    pub fn cursor(self) -> Cursor<'input> {
        Cursor::new(self.iter.as_str(), self.mark)
    }
}

impl<'input> Iterator for IterIndices<'input> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((i, ch)) => {
                match ch {
                    '\n' => self.mark = self.mark.newline(),
                    _ => self.mark = self.mark + Mark::new(0, 1),
                }
                Some((i, ch))
            }
            None => None,
        }
    }
}
