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

impl<T: 'static + Clone> IntoIterator for Expected<T> {
    type Item = T;

    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Expected::One(i) => Iter::One(std::iter::once(i)),
            Expected::Many(i) => Iter::Many(i.iter()),
        }
    }
}

pub enum Iter<T: 'static> {
    One(std::iter::Once<T>),
    Many(std::slice::Iter<'static, T>),
}

impl<T: 'static + Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::One(i) => i.next(),
            Iter::Many(i) => i.next().map(|i| i.clone()),
        }
    }
}
