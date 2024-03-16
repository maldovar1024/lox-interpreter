use std::{collections::HashMap, mem};

use crate::{
    error::{IResult, RuntimeError},
    value::Value,
};

#[derive(Default)]
pub(crate) struct Environment {
    values: HashMap<String, Value>,
    pub(crate) enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub(crate) fn start_scope(&mut self) {
        self.enclosing = Some(Box::new(mem::take(self)))
    }

    pub(crate) fn end_scope(&mut self) {
        let next = mem::take(&mut self.enclosing);
        *self = *next.unwrap();
    }

    pub(crate) fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_owned(), value);
    }

    pub(crate) fn assign(&mut self, name: &str, value: Value) -> IResult<()> {
        let mut environment = Some(self);

        while let Some(env) = environment {
            match env.values.get_mut(name) {
                Some(v) => {
                    *v = value;
                    return Ok(());
                }
                None => environment = env.enclosing.as_deref_mut(),
            }
        }

        Err(RuntimeError::UndefinedVariable {
            name: name.to_owned(),
        }
        .to_box())
    }

    pub(crate) fn get(&self, name: &str) -> IResult<Value> {
        let mut environment = Some(self);

        while let Some(env) = environment {
            match env.values.get(name) {
                Some(v) => return Ok(v.to_owned()),
                None => environment = env.enclosing.as_deref(),
            }
        }

        Err(RuntimeError::UndefinedVariable {
            name: name.to_owned(),
        }
        .to_box())
    }
}
