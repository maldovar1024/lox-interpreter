use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {} column {}", self.line, self.column)
    }
}

#[derive(Debug, Clone, Copy)]
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
    pub fn extends_with(mut self, end: &Self) -> Self {
        self.end = end.end;
        self
    }

    pub fn extends_with_pos(mut self, end: Position) -> Self {
        self.end = end;
        self
    }

    pub fn dummy() -> Self {
        Self {
            start: Position { line: 0, column: 0 },
            end: Position { line: 0, column: 0 },
        }
    }
}
