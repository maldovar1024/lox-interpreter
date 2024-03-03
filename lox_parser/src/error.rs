use thiserror::Error;

use crate::{position::Position, token::TokenType};

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("{1}: unexpected token `{0}`")]
    UnexpectedToken(TokenType, Position),
    #[error("{pos}: expect {expected}, found `{found}`")]
    ExpectStructure {
        expected: &'static str,
        pos: Position,
        found: TokenType,
    },
}

pub type PResult<T> = Result<T, Box<ParserError>>;
