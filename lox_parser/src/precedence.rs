use crate::token::{Keyword, TokenType};

#[derive(Debug, Clone, Copy)]
pub(crate) enum Operator {
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
    None,
    Or,
    Plus,
    Prefix,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Fixity {
    Left,
    Right,
}

impl Operator {
    fn fixity(self) -> Fixity {
        Fixity::Left
    }

    fn precedence(self) -> u8 {
        match self {
            Operator::Prefix => 14,
            Operator::Multiply | Operator::Divide => 13,
            Operator::Minus | Operator::Plus => 12,
            Operator::Greater
            | Operator::GreaterEqual
            | Operator::Less
            | Operator::LessEqual
            | Operator::NotEqual
            | Operator::Equal => 11,
            Operator::And => 10,
            Operator::Or => 9,
            Operator::None => 0,
        }
    }

    pub(crate) fn is_precedent_than(self, left_op: Operator) -> bool {
        match self.precedence().cmp(&left_op.precedence()) {
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => self.fixity() == Fixity::Right,
            std::cmp::Ordering::Greater => true,
        }
    }

    pub(crate) fn from_token(token_type: &TokenType) -> Option<Self> {
        Some(match token_type {
            TokenType::BangEqual => Operator::NotEqual,
            TokenType::EqualEqual => Operator::Equal,
            TokenType::Greater => Operator::Greater,
            TokenType::GreaterEqual => Operator::GreaterEqual,
            TokenType::Less => Operator::Less,
            TokenType::LessEqual => Operator::LessEqual,
            TokenType::Minus => Operator::Minus,
            TokenType::Plus => Operator::Plus,
            TokenType::Slash => Operator::Divide,
            TokenType::Star => Operator::Multiply,
            TokenType::Keyword(Keyword::And) => Operator::And,
            TokenType::Keyword(Keyword::Or) => Operator::Or,
            _ => return None,
        })
    }
}