use error::IResult;
use interpreter::Interpreter;
use lox_parser::ast::{expr::{Expr, Value}, visit::Visitor};

mod interpreter;
pub mod error;

pub fn interpret(expr: &Expr) -> IResult<Value> {
    (Interpreter {}).visit_expr(expr)
}