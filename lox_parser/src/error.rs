use thiserror::Error;

use crate::{span::Span, token::TokenType};

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("{1}: unexpected token `{0}`")]
    UnexpectedToken(TokenType, Span),
    #[error("{span}: expect {expected}, found `{found}`")]
    ExpectStructure {
        expected: &'static str,
        found: TokenType,
        span: Span,
    },
}

pub type PResult<T> = Result<T, Box<ParserError>>;
