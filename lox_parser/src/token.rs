use std::fmt::Display;

use crate::position::Position;
use phf::phf_map;

macro_rules! keywords {
    ($(($expr: expr, $name: ident)),+) => {
        #[derive(Debug, Clone, Copy)]
        pub(crate) enum Keyword {
            $($name,)+
        }

         pub(crate) static KEY_WORDS_MAP: phf::Map<&'static str, Keyword> = phf_map! {
            $($expr => Keyword::$name,)+
        };

        impl Display for Keyword {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Keyword::$name => write!(f, $expr),)+
                }
            }
        }
    };
}

keywords!(
    ("and", And),
    ("class", Class),
    ("else", Else),
    ("false", False),
    ("for", For),
    ("fun", Fun),
    ("if", If),
    ("nil", Nil),
    ("or", Or),
    ("print", Print),
    ("return", Return),
    ("super", Super),
    ("this", This),
    ("true", True),
    ("var", Var),
    ("while", While)
);

#[derive(Debug)]
pub(crate) enum Literal {
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub(crate) enum TokenType {
    Bang,
    BangEqual,
    Comma,
    Dot,
    Eof,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Identifier(String),
    Keyword(Keyword),
    LeftBrace,
    LeftParen,
    Less,
    LessEqual,
    Literal(Literal),
    Minus,
    Plus,
    RightBrace,
    RightParen,
    Semicolon,
    Slash,
    Star,
}

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) position: Position,
}
