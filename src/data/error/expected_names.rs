#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedNames {
    One(&'static str),
    Many(&'static [&'static str]),
}

impl Into<ExpectedNames> for &'static str {
    fn into(self) -> ExpectedNames {
        ExpectedNames::One(self)
    }
}

impl Into<ExpectedNames> for &'static [&'static str] {
    fn into(self) -> ExpectedNames {
        match (self.get(0), self.get(1)) {
            (Some(i), None) => ExpectedNames::One(i),
            _ => ExpectedNames::Many(self),
        }
    }
}

impl IntoIterator for ExpectedNames {
    type Item = &'static str;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ExpectedNames::One(i) => Iter::One(std::iter::once(i)),
            ExpectedNames::Many(i) => Iter::Many(i.iter()),
        }
    }
}

pub enum Iter {
    One(std::iter::Once<&'static str>),
    Many(std::slice::Iter<'static, &'static str>),
}

impl Iterator for Iter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::One(i) => i.next(),
            Iter::Many(i) => i.next().map(|i| *i),
        }
    }
}
