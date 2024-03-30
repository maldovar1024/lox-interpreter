use std::{cell::RefCell, collections::HashMap, fmt::Display, ptr, rc::Rc};
use lox_ast::{ClassDecl, FnDecl, IdentTarget, Lit, Variable};

use crate::{
    environment::{Env, Environment},
    error::{IResult, RuntimeError},
    interpreter::Interpreter,
};

pub trait Callable {
    fn arity(&self) -> u8;

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> IResult<Value>;
}

#[derive(Debug)]
pub struct NativeFunction {
    pub name: &'static str,
    pub arity: u8,
    pub fun: fn(&mut Interpreter, Vec<Value>) -> IResult<Value>,
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Callable for NativeFunction {
    fn arity(&self) -> u8 {
        self.arity
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> IResult<Value> {
        (self.fun)(interpreter, arguments)
    }
}

#[derive(Debug)]
pub struct Function {
    pub declaration: FnDecl,
    pub closure: Option<Env>,
}

impl Callable for Function {
    fn arity(&self) -> u8 {
        self.declaration.params.len() as u8
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> IResult<Value> {
        let mut environment =
            Environment::new(self.declaration.num_of_locals, self.closure.clone());
        for (name, value) in self.declaration.params.iter().zip(arguments) {
            environment.assign(name.target.unwrap(), value)
        }
        interpreter.execute_block(&self.declaration.body, environment)
    }
}

#[derive(Debug)]
pub struct Class {
    pub var: Variable,
    pub super_class: Option<Rc<Class>>,
    pub methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(
        class: &ClassDecl,
        super_class: Option<Rc<Class>>,
        environment: Option<Env>,
    ) -> Self {
        let environment = match super_class.clone() {
            Some(super_class) => {
                let mut environment = Environment::new(1, environment);
                environment.assign(
                    IdentTarget {
                        scope_count: 0,
                        index: 0,
                    },
                    Value::Class(super_class),
                );
                Some(Rc::new(environment.into()))
            }
            None => environment,
        };

        Self {
            var: class.var.clone(),
            super_class,
            methods: class
                .methods
                .iter()
                .map(|method| {
                    (
                        method.var.ident.name.to_string(),
                        Function {
                            declaration: method.clone(),
                            closure: environment.clone(),
                        },
                    )
                })
                .collect(),
        }
    }

    #[inline]
    pub fn get_method(&self, name: &str) -> Option<&Function> {
        self.methods.get(name).or_else(|| {
            self.super_class
                .as_ref()
                .and_then(|super_class| super_class.get_method(name))
        })
    }
}

impl Callable for Rc<Class> {
    fn arity(&self) -> u8 {
        self.get_method("init").map(|m| m.arity()).unwrap_or(0)
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> IResult<Value> {
        let instance = Rc::new(RefCell::new(Instance {
            class: Rc::clone(self),
            fields: Default::default(),
        }));

        if let Some(initializer) = self.get_method("init") {
            if let Err(e) =
                Instance::bind_method(instance.clone(), initializer).call(interpreter, arguments)
            {
                if let RuntimeError::Return(span, value) = *e {
                    if !matches!(value, Value::Nil) {
                        return Err(RuntimeError::ReturnInConstructor(span).to_box());
                    }
                } else {
                    return Err(e.to_box());
                }
            }
        }

        Ok(Value::Instance(instance))
    }
}

#[derive(Debug)]
pub struct Instance {
    class: Rc<Class>,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn get(instance: Rc<RefCell<Self>>, field: &str) -> IResult<Value> {
        let this = instance.borrow();
        match this.fields.get(field) {
            Some(value) => Ok(value.clone()),
            None => match this.class.get_method(field) {
                Some(method) => Ok(Value::Function(Rc::new(Self::bind_method(
                    instance.clone(),
                    method,
                )))),
                None => Err(Box::new(RuntimeError::UndefinedField {
                    field: field.to_string(),
                })),
            },
        }
    }

    pub fn bind_method(instance: Rc<RefCell<Self>>, method: &Function) -> Function {
        let mut closure = Environment::new(1, method.closure.clone());
        closure.assign(
            IdentTarget {
                scope_count: 0,
                index: 0,
            },
            Value::Instance(instance),
        );
        Function {
            declaration: method.declaration.clone(),
            closure: Some(Rc::new(closure.into())),
        }
    }

    pub fn set(&mut self, field: String, value: Value) {
        self.fields.insert(field, value);
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    NativeFunction(Rc<NativeFunction>),
    Function(Rc<Function>),
    Class(Rc<Class>),
    Instance(Rc<RefCell<Instance>>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => n1 == n2,
            (Self::String(s1), Self::String(s2)) => s1 == s2,
            (Self::Bool(b1), Self::Bool(b2)) => b1 == b2,
            (Self::NativeFunction(f1), Self::NativeFunction(f2)) => f1 == f2,
            (Self::Function(f1), Self::Function(f2)) => ptr::eq(f1, f2),
            (Self::Class(f1), Self::Class(f2)) => ptr::eq(f1, f2),
            (Self::Instance(f1), Self::Instance(f2)) => ptr::eq(f1, f2),
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Number(num) => *num != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Bool(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Nil => "nil",
            Value::NativeFunction(_) => "native function",
            Value::Function(_) => "function",
            Value::Class(_) => "class",
            Value::Instance(_) => "instance",
        }
    }
}

impl From<Lit> for Value {
    fn from(value: Lit) -> Self {
        match value {
            Lit::Number(n) => Value::Number(n),
            Lit::String(s) => Value::String(s),
            Lit::Bool(b) => Value::Bool(b),
            Lit::Nil => Value::Nil,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Nil => write!(f, "nil"),
            Value::NativeFunction(fun) => write!(f, "<native function {}>", fun.name),
            Value::Function(fun) => write!(f, "<function {}>", fun.declaration.var),
            Value::Class(class) => write!(f, "<class {}>", class.var),
            Value::Instance(instance) => write!(f, "<{} instance>", instance.borrow().class.var),
        }
    }
}
