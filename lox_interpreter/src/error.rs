use lox_parser::ast::expr::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("TypeError: expected `{expected}`, found `{found}")]
    TypeError {
        expected: &'static str,
        found: &'static str,
    },
}

pub type IResult<T> = Result<T, Box<RuntimeError>>;

impl RuntimeError {
    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn type_error(expected: &'static str, found: &Value) -> Box<RuntimeError> {
        RuntimeError::TypeError {
            expected,
            found: found.type_name(),
        }
        .to_box()
    }
}
