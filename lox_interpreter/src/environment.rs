use std::collections::HashMap;

use lox_parser::ast::expr::Value;

use crate::error::{IResult, RuntimeError};

#[derive(Default)]
pub(crate) struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub(crate) fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_owned(), value);
    }

    pub(crate) fn assign(&mut self, name: &str, value: Value) -> IResult<()> {
        match self.values.get_mut(name) {
            Some(v) => {
                *v = value;
                Ok(())
            }
            None => Err(RuntimeError::UndefinedVariable {
                name: name.to_owned(),
            }
            .to_box()),
        }
    }

    pub(crate) fn get(&self, name: &str) -> IResult<Value> {
        match self.values.get(name) {
            Some(v) => Ok(v.to_owned()),
            None => Err(RuntimeError::UndefinedVariable {
                name: name.to_owned(),
            }
            .to_box()),
        }
    }
}
