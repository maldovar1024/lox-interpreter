use std::fmt::Display;

use crate::{
    error::{PResult, ParserError},
    token::{Keyword, Token, TokenType},
};

#[inline(always)]
pub(crate) fn p<T>(x: T) -> Box<T> {
    Box::new(x)
}

#[derive(Debug)]
pub enum BinaryOp {
    And,
    Divide,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    Multiply,
    NotEqual,
    Or,
    Plus,
}

impl BinaryOp {
    pub(crate) fn from_token(token: Token) -> PResult<Self> {
        Ok(match token.token_type {
            TokenType::BangEqual => Self::NotEqual,
            TokenType::EqualEqual => Self::Equal,
            TokenType::Greater => Self::Greater,
            TokenType::GreaterEqual => Self::GreaterEqual,
            TokenType::Keyword(Keyword::And) => Self::And,
            TokenType::Keyword(Keyword::Or) => Self::Or,
            TokenType::Less => Self::Less,
            TokenType::LessEqual => Self::LessEqual,
            TokenType::Minus => Self::Minus,
            TokenType::Plus => Self::Plus,
            TokenType::Slash => Self::Divide,
            TokenType::Star => Self::Multiply,
            t => return Err(p(ParserError::UnexpectedToken(t, token.position))),
        })
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub operator: BinaryOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub enum UnaryOp {
    Negative,
    Not,
}

impl UnaryOp {
    pub(crate) fn from_token(token: Token) -> PResult<Self> {
        Ok(match token.token_type {
            TokenType::Bang => Self::Not,
            TokenType::Minus => Self::Negative,
            t => return Err(Box::new(ParserError::UnexpectedToken(t, token.position))),
        })
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: UnaryOp,
    pub operand: Box<Expr>,
}

#[derive(Debug)]
pub struct Group {
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Group(Group),
    Literal(Value),
}

impl Expr {
    pub(crate) fn group(expr: Expr) -> Expr {
        Expr::Group(Group { expr: p(expr) })
    }

    pub(crate) fn binary(operator: BinaryOp, left: Expr, right: Expr) -> Expr {
        Expr::Binary(BinaryExpr {
            operator,
            left: p(left),
            right: p(right),
        })
    }

    pub(crate) fn unary(operator: UnaryOp, operand: Expr) -> Expr {
        Expr::Unary(UnaryExpr {
            operator,
            operand: p(operand),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Number(num) => *num != 0.0,
            Value::String(s) => s != "",
            Value::Bool(b) => *b,
            Value::Nil => false,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Nil => "nil",
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
        }
    }
}
