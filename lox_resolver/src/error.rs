use lox_parser::span::Span;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("{0}: undefined variable `{1}`")]
    UndefinedVar(Span, String),
    #[error("{pos}: variable `{name}` is defined at {defined_at}")]
    RedefineVar {
        pos: Span,
        name: String,
        defined_at: Span,
    },
    #[error("{0}: unused variable `{1}`")]
    UnusedVar(Span, String),
    #[error("Can't use `return` outside of a function, {0}")]
    InvalidReturn(Span),
    #[error("Can't return value in constructor, {0}")]
    ReturnInConstructor(Span),
    #[error("Can't use `this` outside of a method, {0}")]
    InvalidThis(Span),
    #[error("Can't use `super` outside of a method, {0}")]
    InvalidSuper(Span),
    #[error("Can't use `super` in a class with no superclass, {0}")]
    NotSubClass(Span),
}
