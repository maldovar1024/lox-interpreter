use lox_ast::{BinaryOp, UnaryOp};
use lox_macros::OpCodec;

use crate::{codec::*, error::*, StringSymbol};

#[derive(Debug, OpCodec)]
pub enum Operation {
    LoadNumber(f64),
    LoadString(StringSymbol),
    LoadBool(bool),
    LoadNil,
    Negative,
    Not,
    Plus,
    Minus,
    Multiply,
    Divide,
    And,
    Or,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
}

impl From<BinaryOp> for Operation {
    fn from(value: BinaryOp) -> Self {
        match value {
            BinaryOp::And => Self::And,
            BinaryOp::Divide => Self::Divide,
            BinaryOp::Equal => Self::Equal,
            BinaryOp::Greater => Self::Greater,
            BinaryOp::GreaterEqual => Self::GreaterEqual,
            BinaryOp::Less => Self::Less,
            BinaryOp::LessEqual => Self::LessEqual,
            BinaryOp::Minus => Self::Minus,
            BinaryOp::Multiply => Self::Multiply,
            BinaryOp::NotEqual => Self::NotEqual,
            BinaryOp::Or => Self::Or,
            BinaryOp::Plus => Self::Plus,
        }
    }
}

impl From<UnaryOp> for Operation {
    fn from(value: UnaryOp) -> Self {
        match value {
            UnaryOp::Negative => Self::Negative,
            UnaryOp::Not => Self::Not,
        }
    }
}
