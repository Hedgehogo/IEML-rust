#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Mark {
    pub line: usize,
    pub symbol: usize,
}

impl Mark {
    pub fn new(line: usize, symbol: usize) -> Self {
        Self { line, symbol }
    }

    pub fn enter(&mut self) {
        self.line += 1;
        self.symbol = 0;
    }
}

impl std::ops::Add for Mark {
    type Output = Mark;

    fn add(self, rhs: Self) -> Self::Output {
        match rhs.line {
            0 => Mark::new(self.line, self.symbol + rhs.symbol),
            _ => Mark::new(self.line + rhs.line, 0),
        }
    }
}
