use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lox_parser::ast::ident::{IdentIndex, IdentTarget};

use crate::{
    error::{IResult, RuntimeError},
    value::Value,
};

#[derive(Default, Debug)]
pub struct Environment {
    values: Vec<Value>,
    pub(crate) enclosing: Option<Env>,
}

pub(crate) type Env = Rc<RefCell<Environment>>;

impl Environment {
    pub(crate) fn new(len: IdentIndex, enclosing: Option<Env>) -> Self {
        Self {
            values: vec![Value::Nil; len as usize],
            enclosing,
        }
    }

    pub(crate) fn assign(&mut self, mut target: IdentTarget, value: Value) {
        if target.scope_count == 0 {
            self.values[target.index as usize] = value;
        } else {
            target.scope_count -= 1;
            self.enclosing
                .as_deref()
                .unwrap()
                .borrow_mut()
                .assign(target, value);
        }
    }

    pub(crate) fn get(&self, mut target: IdentTarget) -> Value {
        if target.scope_count == 0 {
            self.values[target.index as usize].clone()
        } else {
            target.scope_count -= 1;
            self.enclosing.as_deref().unwrap().borrow().get(target)
        }
    }
}

#[derive(Default)]
pub(crate) struct GlobalEnvironment {
    values: HashMap<String, Value>,
}

impl GlobalEnvironment {
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
