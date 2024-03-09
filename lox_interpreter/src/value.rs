use lox_parser::ast::expr;

use crate::error::{IResult, RuntimeError};

#[derive(Debug, Clone, PartialEq)]
pub struct Value(expr::Value);

impl From<expr::Value> for Value {
    fn from(value: expr::Value) -> Self {
        Self(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self(expr::Value::Bool(value))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self(expr::Value::Number(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self(expr::Value::String(value))
    }
}

impl Value {
    pub fn get_number(&self) -> IResult<f64> {
        match &self.0 {
            expr::Value::Number(num) => Ok(*num),
            v => Err(RuntimeError::TypeError {
                expected: "number",
                found: v.type_name(),
            }
            .to_box()),
        }
    }

    pub fn get_string(&self) -> IResult<String> {
        match &self.0 {
            expr::Value::String(s) => Ok(s.clone()),
            v => Err(RuntimeError::TypeError {
                expected: "string",
                found: v.type_name(),
            }
            .to_box()),
        }
    }

    pub fn as_bool(&self) -> bool {
        self.0.as_bool()
    }
}
