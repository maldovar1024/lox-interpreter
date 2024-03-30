use std::fmt::Display;

use crate::span::Span;

pub type IdentIndex = u16;

#[derive(Debug, Clone, Copy)]
pub struct IdentTarget {
    pub scope_count: u16,
    pub index: IdentIndex,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    #[inline]
    pub(crate) fn from_name(name: String, span: Span) -> Self {
        Self { name, span }
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub ident: Ident,
    pub target: Option<IdentTarget>,
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident.name)
    }
}

impl From<Ident> for Variable {
    #[inline]
    fn from(ident: Ident) -> Self {
        Self { ident, target: None }
    }
}

impl Variable {
    #[inline]
    pub(crate) fn from_name(name: String, span: Span) -> Self {
        Ident::from_name(name, span).into()
    }
}
