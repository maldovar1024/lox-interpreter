use crate::{
    ast::{visit::Visitor, visit_mut::VisitorMut},
    ast_enum,
    span::{Position, Span},
    token::{Keyword, TokenType},
};

use super::ident::Ident;

#[inline(always)]
pub(crate) fn p<T>(x: T) -> Box<T> {
    Box::new(x)
}

#[derive(Debug, Clone)]
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

impl From<TokenType> for BinaryOp {
    fn from(token_type: TokenType) -> Self {
        match token_type {
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

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub operator: BinaryOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: UnaryOp,
    pub operand: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Ternary {
    pub condition: Box<Expr>,
    pub truthy: Box<Expr>,
    pub falsy: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Lit {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub callee: Box<Expr>,
    pub arguments: Box<[Expr]>,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub ident: Ident,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Get {
    pub object: Box<Expr>,
    pub field: String,
}

ast_enum! {
    pub enum ExprInner {
        visit_binary: Binary(BinaryExpr),
        visit_unary: Unary(UnaryExpr),
        visit_group: Group(Group),
        visit_literal: Literal(Lit),
        visit_ternary: Ternary(Ternary),
        visit_assign: Assign(Assign),
        visit_var: Var(Ident),
        visit_fn_call: FnCall(FnCall),
        visit_get: Get(Get),
    }
}

#[derive(Debug, Clone)]
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

    pub(crate) fn assign(ident: Ident, value: Expr) -> Self {
        Self {
            span: ident.span.extends_with(&value.span),
            expr: ExprInner::Assign(Assign {
                ident,
                value: p(value),
            }),
        }
    }

    pub(crate) fn get(object: Self, field: Ident) -> Self {
        Self {
            span: object.span.extends_with(&field.span),
            expr: ExprInner::Get(Get {
                object: p(object),
                field: field.name,
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

    pub(crate) fn literal(value: Lit, span: Span) -> Self {
        Self {
            expr: ExprInner::Literal(value),
            span,
        }
    }

    pub(crate) fn var(ident: Ident, span: Span) -> Self {
        Self {
            expr: ExprInner::Var(ident),
            span,
        }
    }
}
