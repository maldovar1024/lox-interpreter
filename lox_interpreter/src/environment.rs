use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::{IResult, RuntimeError},
    value::Value,
};

#[derive(Default, Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
    pub(crate) enclosing: Option<Env>,
}

pub(crate) type Env = Rc<RefCell<Environment>>;

impl Environment {
    pub(crate) fn new(enclosing: Env) -> Self {
        Self {
            values: Default::default(),
            enclosing: Some(enclosing),
        }
    }

    pub(crate) fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_owned(), value);
    }

    pub(crate) fn assign(&mut self, name: &str, value: Value) -> IResult<()> {
        match self.values.get_mut(name) {
            Some(v) => {
                *v = value;
                Ok(())
            }
            None => match &mut self.enclosing {
                Some(enclosing) => enclosing.borrow_mut().assign(name, value),
                None => Err(RuntimeError::UndefinedVariable {
                    name: name.to_owned(),
                }
                .to_box()),
            },
        }
    }

    pub(crate) fn get(&self, name: &str) -> IResult<Value> {
        match self.values.get(name) {
            Some(v) => Ok(v.to_owned()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow().get(name),
                None => Err(RuntimeError::UndefinedVariable {
                    name: name.to_owned(),
                }
                .to_box()),
            },
        }
    }
}
