use lexer::Lexer;
use parser::{Parser, ParserResult};

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
mod precedence;
pub mod span;
pub mod token;

pub fn parse(src: &str) -> ParserResult {
    let mut parser = Parser::new(Lexer::new(src));
    parser.parse()
}
