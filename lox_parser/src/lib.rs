use ast::expr::Expr;
use lexer::Lexer;
use parser::Parser;

mod ast;
mod error;
mod lexer;
mod parser;
mod position;
mod token;

pub fn parse(src: &str) -> Option<Expr> {
    let mut parser = Parser::new(Lexer::new(src));
    parser.parse()
}
