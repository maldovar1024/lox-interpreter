use std::fmt::Display;

use crate::span::Span;
use phf::phf_map;

macro_rules! keywords {
    ($($expr: expr => $name: ident),+) => {
        #[derive(Debug, Clone, Copy)]
        pub enum Keyword {
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
    "and" => And,
    "class" => Class,
    "else" => Else,
    "false" => False,
    "for" => For,
    "fun" => Fun,
    "if" => If,
    "nil" => Nil,
    "or" => Or,
    "print" => Print,
    "return" => Return,
    "super" => Super,
    "this" => This,
    "true" => True,
    "var" => Var,
    "while" => While
);

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
}

#[derive(Debug, Clone)]
pub enum TokenType {
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
    Unknown(char),
    UnterminatedComment,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Eof => write!(f, "end of input"),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Identifier(ident) => write!(f, "{ident}"),
            TokenType::Keyword(kw) => write!(f, "{kw}"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Literal(Literal::String(s)) => write!(f, "{s}"),
            TokenType::Literal(Literal::Number(n)) => write!(f, "{n}"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::RightParen => write!(f, ")"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Unknown(c) => write!(f, "{c}"),
            TokenType::UnterminatedComment => write!(f, "unterminated comment"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) span: Span,
}
