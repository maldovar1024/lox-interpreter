use std::{char, str::Chars};

use crate::{
    position::{self, Position},
    token::{Literal, Token, TokenType, KEY_WORDS_MAP},
};

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
    matches!(c, '0'..='9')
}

pub(crate) struct Lexer<'a> {
    chars: Chars<'a>,
    src: &'a str,
    current_position: Position,
    byte_pos: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(src: &'a str) -> Self {
        Self {
            src,
            chars: src.chars(),
            current_position: Position { line: 1, column: 1 },
            byte_pos: 0,
        }
    }

    pub(crate) fn next_token(&mut self) -> Token {
        loop {
            match self.advance() {
                Some(token) => return token,
                None => {}
            }
        }
    }

    fn advance(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.update_byte_pos();
        let start = self.current_position.clone();

        let first_char = match self.bump() {
            Some(c) => c,
            None => return Some(self.yield_token(TokenType::Eof, start)),
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
            '/' => {
                if self.test_and_bump('/') {
                    self.skip_white(|c| c != '\n' && c != '\r');
                    return None;
                }
                TokenType::Slash
            }
            '"' => TokenType::Literal(Literal::String(self.string())),
            '0'..='9' => TokenType::Literal(Literal::Number(self.number())),
            c if is_ident_start(c) => self.identifier(),
            _ => todo!(),
        };

        Some(self.yield_token(token_type, start))
    }

    fn yield_token(&self, token_type: TokenType, position: Position) -> Token {
        Token {
            token_type,
            position,
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

    fn new_line(&mut self) {
        self.current_position.line += 1;
        self.current_position.column = 1;
    }

    fn bump(&mut self) -> Option<char> {
        let mut next = self.chars.next()?;
        match next {
            '\n' => self.new_line(),
            '\r' if self.test_and_bump('\n') => {
                self.new_line();
                next = '\n'
            }
            _ => self.current_position.column += 1,
        }
        Some(next)
    }

    fn test_and_bump(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.bump();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        self.skip_white(is_whitespace);
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn skip_white(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while !self.is_eof() && predicate(self.peek()) {
            self.bump();
        }
    }

    fn string(&mut self) -> String {
        let mut result = String::new();
        loop {
            match self.peek() {
                '\\' => {
                    self.bump();
                    match self.peek() {
                        '\\' => result.push('\\'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        '"' => result.push('"'),
                        _ => todo!(),
                    }
                }
                '"' => break,
                ch => result.push(ch),
            }
            self.bump();
        }
        result
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
