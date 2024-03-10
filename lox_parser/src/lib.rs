use ast::expr::Expr;
use error::PResult;
use lexer::Lexer;
use parser::Parser;

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod position;
mod precedence;
pub mod token;

pub fn parse(src: &str) -> PResult<Expr> {
    let mut parser = Parser::new(Lexer::new(src));
    parser.parse()
}
