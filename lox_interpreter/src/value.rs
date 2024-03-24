use std::{fmt::Display, ptr, rc::Rc};

use lox_parser::ast::{expr::Lit, stmt::FnDecl};

use crate::{
    environment::{Env, Environment},
    error::IResult,
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

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    NativeFunction(Rc<NativeFunction>),
    Function(Rc<Function>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => n1 == n2,
            (Self::String(s1), Self::String(s2)) => s1 == s2,
            (Self::Bool(b1), Self::Bool(b2)) => b1 == b2,
            (Self::NativeFunction(f1), Self::NativeFunction(f2)) => f1 == f2,
            (Self::Function(f1), Self::Function(f2)) => ptr::eq(f1, f2),
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
        }
    }
}

impl From<Lit> for Value {
    fn from(value: lox_parser::ast::expr::Lit) -> Self {
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
            Value::Function(fun) => write!(f, "<function {}>", fun.declaration.ident),
        }
    }
}
