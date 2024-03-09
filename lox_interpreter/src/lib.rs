use error::IResult;
use interpreter::Interpreter;
use lox_parser::ast::{expr::Expr, visit::Visitor};
use value::Value;

mod interpreter;
pub mod error;
pub mod value;

pub fn interpret(expr: &Expr) -> IResult<Value> {
    (Interpreter {}).visit_expr(expr)
}