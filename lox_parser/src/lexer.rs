use std::{char, str::Chars};

use crate::{
    span::{Position, Span},
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

pub struct Lexer<'a> {
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
        match self.skip() {
            Some(token) => return token,
            None => {}
        }

        self.update_byte_pos();
        let start = self.current_position.clone();

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
            '"' => TokenType::Literal(Literal::String(self.string())),
            '0'..='9' => TokenType::Literal(Literal::Number(self.number())),
            c if is_ident_start(c) => self.identifier(),
            c => TokenType::Unknown(c),
        };

        self.yield_token(token_type, start)
    }

    fn yield_token(&self, token_type: TokenType, start: Position) -> Token {
        Token {
            token_type,
            span: Span {
                start,
                end: self.current_position.clone(),
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

    fn new_line(&mut self) {
        self.current_position.line += 1;
        self.current_position.column = 1;
    }

    fn bump(&mut self) -> Option<char> {
        let mut next = self.chars.next()?;
        match next {
            '\n' => self.new_line(),
            '\r' if self.peek() == '\n' => {
                self.chars.next();
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
        let start = self.current_position.clone();

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
