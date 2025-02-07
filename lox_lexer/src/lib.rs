mod span;
mod token;

use std::{char, str::Chars};

use crate::token::KEY_WORDS_MAP;

pub use span::*;
pub use token::*;

const EOF_CHAR: char = '\0';

fn is_whitespace(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r' | ' ')
}

fn is_ident_start(c: char) -> bool {
    matches!(c, '_' | 'a'..='z' | 'A'..='Z')
}

fn is_ident_continue(c: char) -> bool {
    matches!(c, '_' | 'a'..='z' | 'A'..='Z' | '0'..='9')
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
    src: &'a str,
    byte_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            chars: src.chars(),
            byte_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self.skip() {
            return token;
        }

        self.update_byte_pos();
        let start = self.byte_pos as u32;

        let first_char = match self.bump() {
            Some(c) => c,
            None => return self.yield_token(TokenType::Eof, start),
        };

        let token_type = match first_char {
            '!' => {
                if self.test_and_bump('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '=' => {
                if self.test_and_bump('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '>' => {
                if self.test_and_bump('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '<' => {
                if self.test_and_bump('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            ';' => TokenType::Semicolon,
            '/' => TokenType::Slash,
            '?' => TokenType::Question,
            ':' => TokenType::Colon,
            '"' => self.string(),
            '0'..='9' => TokenType::Literal(Literal::Number(self.number())),
            c if is_ident_start(c) => self.identifier(),
            c => TokenType::Unknown(c),
        };

        self.yield_token(token_type, start)
    }

    fn yield_token(&self, token_type: TokenType, start: u32) -> Token {
        Token {
            token_type,
            span: Span {
                start,
                end: self.byte_pos as u32,
            },
        }
    }

    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn peek_next(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF_CHAR)
    }

    fn get_current_pos(&self) -> usize {
        self.src.len() - self.chars.as_str().len()
    }

    fn update_byte_pos(&mut self) {
        self.byte_pos = self.src.len() - self.chars.as_str().len();
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn test_and_bump(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.bump();
            true
        } else {
            false
        }
    }

    fn skip(&mut self) -> Option<Token> {
        loop {
            match self.peek() {
                '/' => {
                    if self.peek_next() == '/' {
                        self.skip_white(|c| c != '\n' && c != '\r');
                    } else if self.peek_next() == '*' {
                        let comments = self.skip_multiline_comment();
                        if comments.is_some() {
                            return comments;
                        }
                    } else {
                        return None;
                    }
                }
                c if is_whitespace(c) => {
                    self.bump();
                }
                _ => return None,
            }
        }
    }

    fn skip_multiline_comment(&mut self) -> Option<Token> {
        let mut level = 1;
        let start = self.byte_pos as u32;

        self.bump();
        self.bump();

        while let Some(c) = self.bump() {
            match c {
                '/' if self.peek() == '*' => {
                    level += 1;
                    self.bump();
                }
                '*' if self.peek() == '/' => {
                    level -= 1;
                    self.bump();
                    if level == 0 {
                        return None;
                    }
                }
                _ => {}
            }
        }

        Some(self.yield_token(TokenType::UnterminatedComment, start))
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn skip_white(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while !self.is_eof() && predicate(self.peek()) {
            self.bump();
        }
    }

    fn string(&mut self) -> TokenType {
        let mut result = String::new();
        while let Some(c) = self.bump() {
            match c {
                '\\' => match self.bump().unwrap_or(EOF_CHAR) {
                    '\\' => result.push('\\'),
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    '"' => result.push('"'),
                    _ => todo!(),
                },
                '"' => return TokenType::Literal(Literal::String(result)),
                ch => result.push(ch),
            }
        }

        TokenType::UnterminatedString
    }

    fn identifier(&mut self) -> TokenType {
        let start = self.byte_pos;
        self.skip_white(is_ident_continue);
        let end = self.get_current_pos();
        let ident = &self.src[start..end];

        match KEY_WORDS_MAP.get(ident) {
            Some(&kw) => TokenType::Keyword(kw),
            None => TokenType::Identifier(ident.to_string()),
        }
    }

    fn number(&mut self) -> f64 {
        let start = self.byte_pos;

        self.skip_white(is_digit);
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.bump();
            self.skip_white(is_digit);
        }

        let end = self.get_current_pos();

        self.src[start..end].parse().unwrap()
    }
}
