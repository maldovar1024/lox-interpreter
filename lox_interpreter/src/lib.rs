use error::IResult;
use interpreter::Interpreter;
use lox_parser::{ast::expr::Value, parser::Ast};

pub mod error;
mod interpreter;

pub fn interpret(ast: &Ast) -> IResult<Value> {
    (Interpreter {}).interpret(ast)
}
