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
    #[error("Undefined variable `{field}`")]
    UndefinedField { field: String },
    #[error("Cannot read field of type {target_type}, reading {field}")]
    InvalidFieldTarget { target_type: &'static str, field: String },
    #[error("{target} is not callable, {span}")]
    NotCallable { target: String, span: Span },
    #[error("Expected {expected} arguments. but got {got}, {span}")]
    ArgumentsNotMatch {
        expected: u8,
        got: usize,
        span: Span,
    },
    #[error("`Return` must be in a function, {0}")]
    Return(Span, Value),
    #[error("Cannot return value in constructor, {0}")]
    ReturnInConstructor(Span),
    #[error("Invalid super class, {0}")]
    InvalidSuperClass(Span),
}

pub type IResult<T> = Result<T, Box<RuntimeError>>;

impl RuntimeError {
    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn type_error(span: Span, expected: &'static str, found: &Value) -> Box<RuntimeError> {
        RuntimeError::TypeError {
            span,
            expected,
            found: found.type_name(),
        }
        .to_box()
    }
}
