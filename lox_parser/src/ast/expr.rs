use std::fmt::Display;

use crate::{
    span::{Position, Span},
    token::{Keyword, TokenType},
};

#[inline(always)]
pub(crate) fn p<T>(x: T) -> Box<T> {
    Box::new(x)
}

#[derive(Debug)]
pub enum BinaryOp {
    And,
    Assign,
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

impl From<TokenType> for BinaryOp {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Equal => Self::Assign,
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
            _ => unreachable!(),
        }
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

impl From<TokenType> for UnaryOp {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Bang => Self::Not,
            TokenType::Minus => Self::Negative,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: UnaryOp,
    pub operand: Box<Expr>,
}

#[derive(Debug)]
pub struct Ternary {
    pub condition: Box<Expr>,
    pub truthy: Box<Expr>,
    pub falsy: Box<Expr>,
}

#[derive(Debug)]
pub struct Group {
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct FnCall {
    pub callee: Box<Expr>,
    pub arguments: Box<[Expr]>,
}

#[derive(Debug)]
pub enum ExprInner {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Ternary(Ternary),
    Group(Group),
    Literal(Value),
    Var(String),
    FnCall(FnCall),
}

#[derive(Debug)]
pub struct Expr {
    pub expr: ExprInner,
    pub span: Span,
}

impl Expr {
    pub(crate) fn group(expr: Self, start: Position, end: Position) -> Self {
        Self {
            expr: ExprInner::Group(Group { expr: p(expr) }),
            span: Span { start, end },
        }
    }

    pub(crate) fn binary(operator: BinaryOp, left: Self, right: Self) -> Self {
        Self {
            span: left.span.extends_with(&right.span),
            expr: ExprInner::Binary(BinaryExpr {
                operator,
                left: p(left),
                right: p(right),
            }),
        }
    }

    pub(crate) fn unary(operator: UnaryOp, op_span: Span, operand: Self) -> Self {
        Self {
            span: op_span.extends_with(&operand.span),
            expr: ExprInner::Unary(UnaryExpr {
                operator,
                operand: p(operand),
            }),
        }
    }

    pub(crate) fn ternary(condition: Self, truthy: Self, falsy: Self) -> Self {
        Self {
            span: condition.span.extends_with(&falsy.span),
            expr: ExprInner::Ternary(Ternary {
                condition: p(condition),
                truthy: p(truthy),
                falsy: p(falsy),
            }),
        }
    }

    pub(crate) fn literal(value: Value, span: Span) -> Self {
        Self {
            expr: ExprInner::Literal(value),
            span,
        }
    }

    pub(crate) fn var(ident: String, span: Span) -> Self {
        Self {
            expr: ExprInner::Var(ident),
            span,
        }
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
