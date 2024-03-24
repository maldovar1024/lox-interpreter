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
    #[error("{0}:iInvalid left value in assignment")]
    InvalidLeftValue(Span),
}
