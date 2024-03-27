use std::{cell::RefCell, collections::HashMap, fmt::Display, ptr, rc::Rc};

use lox_parser::ast::{
    expr::Lit,
    ident::Ident,
    stmt::{ClassDecl, FnDecl},
};

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
    pub ident: Ident,
    pub methods_map: HashMap<String, usize>,
    pub methods: Box<[Function]>,
}

impl Class {
    pub fn new(class: ClassDecl, environment: Option<Env>) -> Self {
        let mut methods_map = HashMap::with_capacity(class.methods.len());
        let methods = class
            .methods
            .into_vec()
            .into_iter()
            .enumerate()
            .map(|(idx, method)| {
                methods_map.insert(method.ident.name.to_string(), idx);
                Function {
                    declaration: method,
                    closure: environment.clone(),
                }
            })
            .collect();

        Self {
            ident: class.ident,
            methods,
            methods_map,
        }
    }
}

impl Callable for Rc<Class> {
    fn arity(&self) -> u8 {
        0
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> IResult<Value> {
        Ok(Value::Instance(Rc::new(RefCell::new(Instance {
            class: Rc::clone(self),
            fields: Default::default(),
        }))))
    }
}

#[derive(Debug, Clone)]
pub struct Method {
    class: Rc<Class>,
    index: usize,
}

impl Method {
    pub fn get_method(&self) -> &Function {
        &self.class.methods[self.index]
    }
}

impl Callable for Method {
    fn arity(&self) -> u8 {
        self.get_method().arity()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> IResult<Value> {
        self.get_method().call(interpreter, arguments)
    }
}

#[derive(Debug)]
pub struct Instance {
    class: Rc<Class>,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn get(&self, field: &str) -> IResult<Value> {
        match self.fields.get(field) {
            Some(value) => Ok(value.clone()),
            None => match self.class.methods_map.get(field) {
                Some(&index) => Ok(Value::Method(Method {
                    class: self.class.clone(),
                    index,
                })),
                None => Err(Box::new(RuntimeError::UndefinedField {
                    field: field.to_string(),
                })),
            },
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
    Method(Method),
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
            (Self::Method(m1), Self::Method(m2)) => {
                ptr::eq(&m1.class, &m2.class) && m1.index == m2.index
            }
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
            Value::Method(_) => "method",
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
            Value::Class(class) => write!(f, "<class {}>", class.ident),
            Value::Instance(instance) => write!(f, "<{} instance>", instance.borrow().class.ident),
            Value::Method(method) => write!(
                f,
                "<method {}.{}>",
                method.class.ident,
                method.get_method().declaration.ident
            ),
        }
    }
}
