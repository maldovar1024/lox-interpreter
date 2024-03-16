use error::IResult;
use interpreter::Interpreter;
use lox_parser::parser::Ast;
use value::Value;

mod environment;
pub mod error;
mod interpreter;
mod value;

pub fn interpret(ast: &Ast) -> IResult<Value> {
    Interpreter::new().interpret(ast)
}
