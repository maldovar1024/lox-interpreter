use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Position {
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
