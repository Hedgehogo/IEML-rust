#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Mark {
    pub line: usize,
    pub symbol: usize,
}

impl Mark {
    pub fn enter(&mut self) {
        self.line += 1;
        self.symbol = 0;
    }
}