use lox_lexer::Lexer;
use parser::{Parser, ParserResult};

pub mod error;
pub mod parser;
mod precedence;

pub fn parse(src: &str) -> ParserResult {
    let mut parser = Parser::new(Lexer::new(src));
    parser.parse()
}
