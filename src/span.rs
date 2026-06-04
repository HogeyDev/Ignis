#[derive(Debug, Copy, Clone)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, x: usize) -> bool {
        self.start <= x && x <= self.end
    }

    pub fn merge(&self, other: &Span) -> Self {
        Self {
            start: std::cmp::min(self.start, other.start),
            end: std::cmp::min(self.end, other.end),
        }
    }
}
