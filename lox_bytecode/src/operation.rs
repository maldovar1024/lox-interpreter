use lox_ast::{BinaryOp, UnaryOp};

pub enum Operation {
    LoadNumber(f64),
    LoadString(u8),
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
