use lox_parser::span::Span;
use thiserror::Error;

use crate::value::Value;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("TypeError: expected `{expected}`, found `{found}")]
    TypeError {
        span: Span,
        expected: &'static str,
        found: &'static str,
    },
    #[error("Undefined variable `{name}`")]
    UndefinedVariable { name: String },
    #[error("Invalid left value in assignment, {0}")]
    InvalidLeftValue(Span),
}

pub type IResult<T> = Result<T, Box<RuntimeError>>;

impl RuntimeError {
    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn type_error(span: &Span, expected: &'static str, found: &Value) -> Box<RuntimeError> {
        RuntimeError::TypeError {
            span: span.clone(),
            expected,
            found: found.type_name(),
        }
        .to_box()
    }
}
