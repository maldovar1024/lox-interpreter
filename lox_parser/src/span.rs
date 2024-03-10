use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {} column {}", self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "from {} to {}", self.start, self.end)
    }
}

impl Span {
    pub fn extends_with(&self, end: &Self) -> Self {
        Self {
            start: self.start.clone(),
            end: end.end.clone(),
        }
    }
}
