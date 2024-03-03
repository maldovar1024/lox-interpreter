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
    operator: BinaryOp,
    left: Box<Expr>,
    right: Box<Expr>,
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
    operator: UnaryOp,
    operand: Box<Expr>,
}

#[derive(Debug)]
pub struct Group {
    expr: Box<Expr>,
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

#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
