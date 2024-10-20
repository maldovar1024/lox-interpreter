use crate::{ast_enum, visit::Visitor, visit_mut::VisitorMut};
use lox_lexer::{Keyword, Position, Span, TokenType};

use super::ident::{Ident, Variable};

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
    pub left: Expr,
    pub right: Expr,
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
    pub operand: Expr,
}

#[derive(Debug, Clone)]
pub struct Ternary {
    pub condition: Expr,
    pub truthy: Expr,
    pub falsy: Expr,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub span: Span,
    pub expr: Expr,
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
    pub callee: Expr,
    pub arguments: Box<[Expr]>,
    pub end: Position,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub var: Variable,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct Get {
    pub object: Expr,
    pub field: Ident,
}

#[derive(Debug, Clone)]
pub struct Set {
    pub target: Get,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct Super {
    pub var: Variable,
    pub method: Ident,
}

ast_enum! {
    pub enum Expr {
        visit_binary: Binary(Box<BinaryExpr>),
        visit_unary: Unary(Box<UnaryExpr>),
        visit_group: Group(Box<Group>),
        visit_literal: Literal(Box<Literal>),
        visit_ternary: Ternary(Box<Ternary>),
        visit_assign: Assign(Box<Assign>),
        visit_var: Var(Box<Variable>),
        visit_fn_call: FnCall(Box<FnCall>),
        visit_get: Get(Box<Get>),
        visit_set: Set(Box<Set>),
        visit_super: Super(Box<Super>),
    }
}

impl Expr {
    pub fn get_span(&self) -> Span {
        match self {
            Expr::Binary(binary) => binary
                .left
                .get_span()
                .extends_with(&binary.right.get_span()),
            Expr::Unary(unary) => unary.op_span.extends_with(&unary.operand.get_span()),
            Expr::Group(group) => group.span,
            Expr::Literal(literal) => literal.span,
            Expr::Ternary(ternary) => ternary
                .condition
                .get_span()
                .extends_with(&ternary.falsy.get_span()),
            Expr::Assign(assign) => assign.var.ident.span.extends_with(&assign.value.get_span()),
            Expr::Var(var) => var.ident.span,
            Expr::FnCall(fn_call) => fn_call.callee.get_span().extends_with_pos(fn_call.end),
            Expr::Get(get) => get.object.get_span().extends_with(&get.field.span),
            Expr::Set(set) => set
                .target
                .object
                .get_span()
                .extends_with(&set.value.get_span()),
            Expr::Super(su) => su.var.ident.span.extends_with(&su.method.span),
        }
    }

    pub fn group(expr: Self, start: Position, end: Position) -> Self {
        Self::Group(Box::new(Group {
            expr,
            span: Span { start, end },
        }))
    }

    pub fn binary(operator: BinaryOp, left: Self, right: Self) -> Self {
        Self::Binary(Box::new(BinaryExpr {
            operator,
            left,
            right,
        }))
    }

    pub fn assign(var: Variable, value: Expr) -> Self {
        Self::Assign(Box::new(Assign { var, value }))
    }

    pub fn get(object: Self, field: Ident) -> Self {
        Self::Get(Box::new(Get { object, field }))
    }

    pub fn set(get: Get, value: Expr) -> Self {
        Self::Set(Box::new(Set {
            target: get,
            value,
        }))
    }

    pub fn unary(operator: UnaryOp, op_span: Span, operand: Self) -> Self {
        Self::Unary(Box::new(UnaryExpr {
            op_span,
            operator,
            operand,
        }))
    }

    pub fn ternary(condition: Self, truthy: Self, falsy: Self) -> Self {
        Self::Ternary(Box::new(Ternary {
            condition,
            truthy,
            falsy,
        }))
    }

    pub fn literal(value: Lit, span: Span) -> Self {
        Self::Literal(Box::new(Literal { span, value }))
    }
}
