use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expected<T: 'static> {
    One(T),
    Many(&'static [T]),
}

impl<T: 'static> Expected<T> {
    pub fn len(&self) -> usize {
        match self {
            Expected::One(_) => 1,
            Expected::Many(i) => i.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Expected::One(_) => false,
            Expected::Many(i) => i.is_empty(),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }

    pub fn display<F>(
        &self,
        f: &mut fmt::Formatter<'_>,
        one: &str,
        many: &str,
        display: F,
    ) -> fmt::Result
    where
        F: Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
    {
        match self {
            Expected::One(i) => {
                write!(f, ", expected {} ", one)?;
                display(i, f)?;
            }
            Expected::Many(i) => {
                let mut iter = i.iter();
                if let Some(i) = iter.next() {
                    write!(f, ", expected {}: ", many)?;
                    display(i, f)?;
                    for i in iter {
                        write!(f, ", ")?;
                        display(i, f)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: 'static> From<T> for Expected<T> {
    fn from(value: T) -> Self {
        Expected::One(value)
    }
}

impl<T: 'static> From<&'static [T]> for Expected<T> {
    fn from(value: &'static [T]) -> Self {
        Expected::Many(value)
    }
}

impl<T: 'static> std::ops::Index<usize> for Expected<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Expected::One(i) => i,
            Expected::Many(i) => i.index(index),
        }
    }
}

impl<'data, T: 'static> IntoIterator for &'data Expected<T> {
    type Item = &'data T;

    type IntoIter = Iter<'data, T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Expected::One(i) => Iter::One(std::iter::once(i)),
            Expected::Many(i) => Iter::Many(i.iter()),
        }
    }
}

pub enum Iter<'data, T: 'static> {
    One(std::iter::Once<&'data T>),
    Many(std::slice::Iter<'static, T>),
}

impl<'data, T: 'static> Iterator for Iter<'data, T> {
    type Item = &'data T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::One(i) => i.next(),
            Iter::Many(i) => i.next(),
        }
    }
}
