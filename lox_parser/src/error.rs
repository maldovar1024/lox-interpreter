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
    #[error("{0}: too many parameters")]
    TooManyParameters(Span),
}

impl ParserError {
    pub(crate) fn expect_structure(
        expected: &'static str,
        found: TokenType,
        span: Span,
    ) -> Box<Self> {
        Box::new(Self::ExpectStructure {
            expected,
            found,
            span,
        })
    }
}

pub type PResult<T> = Result<T, Box<ParserError>>;
