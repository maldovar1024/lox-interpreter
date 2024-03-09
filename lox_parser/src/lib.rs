use ast::expr::Expr;
use lexer::Lexer;
use parser::Parser;

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod position;
pub mod token;

pub fn parse(src: &str) -> Option<Expr> {
    let mut parser = Parser::new(Lexer::new(src));
    parser.parse()
}
