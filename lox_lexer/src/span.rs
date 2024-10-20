use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "from {} to {}", self.start, self.end)
    }
}

impl Span {
    #[inline(always)]
    pub fn extends_with(mut self, end: &Self) -> Self {
        self.end = end.end;
        self
    }

    #[inline(always)]
    pub fn extends_with_pos(mut self, end: u32) -> Self {
        self.end = end;
        self
    }

    #[inline(always)]
    pub fn dummy() -> Self {
        Self::default()
    }
}
