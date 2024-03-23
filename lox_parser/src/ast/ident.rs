use std::fmt::Display;

pub type IdentIndex = u16;

#[derive(Debug, Clone, Copy)]
pub struct IdentTarget {
    pub scope_count: u16,
    pub index: IdentIndex,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub target: Option<IdentTarget>,
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Ident {
    pub(crate) fn from_name(name: String) -> Self {
        Self {
            name,
            target: None,
        }
    }
}
