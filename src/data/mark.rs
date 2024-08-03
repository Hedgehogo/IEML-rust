//! Type definition [`Mark`]

/// Structure storing row and column number in the document
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Mark {
    /// Line number starting from zero
    pub line: usize,
    /// Column number starting from zero
    pub symbol: usize,
}

impl Mark {
    /// Creates a [`Mark`] from row and column numbers
    ///
    /// # Arguments
    /// * `line` Line number starting from zero.
    /// * `symbol` Column number starting from zero.
    pub fn new(line: usize, symbol: usize) -> Self {
        Self { line, symbol }
    }

    /// Gets the `Mark` that would be assigned to the position received by pressing the `Enter` key
    pub fn newline(self) -> Self {
        Self::new(self.line + 1, 0)
    }
}

impl std::ops::Add for Mark {
    type Output = Mark;

    fn add(self, rhs: Self) -> Self::Output {
        match rhs.line {
            0 => Mark::new(self.line, self.symbol + rhs.symbol),
            _ => Mark::new(self.line + rhs.line, rhs.symbol),
        }
    }
}
