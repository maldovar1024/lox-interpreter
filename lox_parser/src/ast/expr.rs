use crate::{
    ast::{visit::Visitor, visit_mut::VisitorMut},
    ast_enum,
    span::{Position, Span},
    token::{Keyword, TokenType},
};

use super::ident::{Ident, Variable};

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
    pub op_span: Span,
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
    pub span: Span,
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
pub struct Literal {
    pub span: Span,
    pub value: Lit,
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub callee: Box<Expr>,
    pub arguments: Box<[Expr]>,
    pub end: Position,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub var: Variable,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Get {
    pub object: Box<Expr>,
    pub field: Ident,
}

#[derive(Debug, Clone)]
pub struct Set {
    pub target: Get,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Super {
    pub var: Variable,
    pub method: Ident,
}

ast_enum! {
    pub enum Expr {
        visit_binary: Binary(BinaryExpr),
        visit_unary: Unary(UnaryExpr),
        visit_group: Group(Group),
        visit_literal: Literal(Literal),
        visit_ternary: Ternary(Ternary),
        visit_assign: Assign(Assign),
        visit_var: Var(Variable),
        visit_fn_call: FnCall(FnCall),
        visit_get: Get(Get),
        visit_set: Set(Set),
        visit_super: Super(Super),
    }
}

impl Expr {
    pub fn get_span(&self) -> Span {
        match self {
            Expr::Binary(BinaryExpr { left, right, .. }) => {
                left.get_span().extends_with(&right.get_span())
            }
            Expr::Unary(UnaryExpr {
                op_span, operand, ..
            }) => op_span.extends_with(&operand.get_span()),
            Expr::Group(group) => group.span,
            Expr::Literal(literal) => literal.span,
            Expr::Ternary(Ternary {
                condition, falsy, ..
            }) => condition.get_span().extends_with(&falsy.get_span()),
            Expr::Assign(Assign { var, value }) => var.ident.span.extends_with(&value.get_span()),
            Expr::Var(var) => var.ident.span,
            Expr::FnCall(FnCall { callee,  end, ..}) => callee.get_span().extends_with_pos(*end),
            Expr::Get(Get { object, field }) => object.get_span().extends_with(&field.span),
            Expr::Set(Set { target, value }) => target.object.get_span().extends_with(&value.get_span()),
            Expr::Super(Super { var, method }) => var.ident.span.extends_with(&method.span),
        }
    }

    pub(crate) fn group(expr: Self, start: Position, end: Position) -> Self {
        Self::Group(Group {
            expr: p(expr),
            span: Span { start, end },
        })
    }

    pub(crate) fn binary(operator: BinaryOp, left: Self, right: Self) -> Self {
        Self::Binary(BinaryExpr {
            operator,
            left: p(left),
            right: p(right),
        })
    }

    pub(crate) fn assign(var: Variable, value: Expr) -> Self {
        Self::Assign(Assign {
            var,
            value: p(value),
        })
    }

    pub(crate) fn get(object: Self, field: Ident) -> Self {
        Self::Get(Get {
            object: p(object),
            field,
        })
    }

    pub(crate) fn set(get: Get, value: Expr) -> Self {
        Self::Set(Set {
            target: get,
            value: p(value),
        })
    }

    pub(crate) fn unary(operator: UnaryOp, op_span: Span, operand: Self) -> Self {
        Self::Unary(UnaryExpr {
            op_span,
            operator,
            operand: p(operand),
        })
    }

    pub(crate) fn ternary(condition: Self, truthy: Self, falsy: Self) -> Self {
        Self::Ternary(Ternary {
            condition: p(condition),
            truthy: p(truthy),
            falsy: p(falsy),
        })
    }

    pub(crate) fn literal(value: Lit, span: Span) -> Self {
        Self::Literal(Literal { span, value })
    }
}
