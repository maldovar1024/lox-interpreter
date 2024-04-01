use crate::{ast_enum, visit::Visitor, visit_mut::VisitorMut};
use lox_lexer::{Keyword, Position, Span, TokenType};

use super::ident::{Ident, Variable};

#[inline(always)]
pub fn p<T>(x: T) -> Box<T> {
    Box::new(x)
}

#[derive(Debug, Clone, Copy)]
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

impl BinaryExpr {
    #[inline]
    pub fn get_span(&self) -> Span {
        self.left.get_span().extends_with(&self.right.get_span())
    }
}

#[derive(Debug, Clone, Copy)]
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

impl UnaryExpr {
    #[inline]
    pub fn get_span(&self) -> Span {
        self.op_span.extends_with(&self.operand.get_span())
    }
}

#[derive(Debug, Clone)]
pub struct Ternary {
    pub condition: Box<Expr>,
    pub truthy: Box<Expr>,
    pub falsy: Box<Expr>,
}

impl Ternary {
    #[inline]
    pub fn get_span(&self) -> Span {
        self.condition
            .get_span()
            .extends_with(&self.falsy.get_span())
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    pub span: Span,
    pub expr: Box<Expr>,
}

impl Group {
    #[inline]
    pub fn get_span(&self) -> Span {
        self.span
    }
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

impl Literal {
    #[inline]
    pub fn get_span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub callee: Box<Expr>,
    pub arguments: Box<[Expr]>,
    pub end: Position,
}

impl FnCall {
    #[inline]
    pub fn get_span(&self) -> Span {
        self.callee.get_span().extends_with_pos(self.end)
    }
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub var: Variable,
    pub value: Box<Expr>,
}

impl Assign {
    #[inline]
    pub fn get_span(&self) -> Span {
        self.var.ident.span.extends_with(&self.value.get_span())
    }
}

#[derive(Debug, Clone)]
pub struct Get {
    pub object: Box<Expr>,
    pub field: Ident,
}

impl Get {
    #[inline]
    pub fn get_span(&self) -> Span {
        self.object.get_span().extends_with(&self.field.span)
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub target: Get,
    pub value: Box<Expr>,
}

impl Set {
    #[inline]
    pub fn get_span(&self) -> Span {
        self.target
            .object
            .get_span()
            .extends_with(&self.value.get_span())
    }
}

#[derive(Debug, Clone)]
pub struct Super {
    pub var: Variable,
    pub method: Ident,
}

impl Super {
    #[inline]
    pub fn get_span(&self) -> Span {
        self.var.ident.span.extends_with(&self.method.span)
    }
}

macro_rules! expr {
    (pub enum $enum_name: ident {$($walker: ident: $name: ident($ty: ty)),+ $(,)?}) => {
        ast_enum! {
            pub enum $enum_name {
                $($walker: $name($ty),)+
            }
        }

        impl $enum_name {
            pub fn get_span(&self) -> Span {
                match self {
                    $(Self::$name(variant) => variant.get_span()),+
                }
            }
        }
    };
}

expr! {
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
    pub fn group(expr: Self, start: Position, end: Position) -> Self {
        Self::Group(Group {
            expr: p(expr),
            span: Span { start, end },
        })
    }

    pub fn binary(operator: BinaryOp, left: Self, right: Self) -> Self {
        Self::Binary(BinaryExpr {
            operator,
            left: p(left),
            right: p(right),
        })
    }

    pub fn assign(var: Variable, value: Expr) -> Self {
        Self::Assign(Assign {
            var,
            value: p(value),
        })
    }

    pub fn get(object: Self, field: Ident) -> Self {
        Self::Get(Get {
            object: p(object),
            field,
        })
    }

    pub fn set(get: Get, value: Expr) -> Self {
        Self::Set(Set {
            target: get,
            value: p(value),
        })
    }

    pub fn unary(operator: UnaryOp, op_span: Span, operand: Self) -> Self {
        Self::Unary(UnaryExpr {
            op_span,
            operator,
            operand: p(operand),
        })
    }

    pub fn ternary(condition: Self, truthy: Self, falsy: Self) -> Self {
        Self::Ternary(Ternary {
            condition: p(condition),
            truthy: p(truthy),
            falsy: p(falsy),
        })
    }

    pub fn literal(value: Lit, span: Span) -> Self {
        Self::Literal(Literal { span, value })
    }
}
