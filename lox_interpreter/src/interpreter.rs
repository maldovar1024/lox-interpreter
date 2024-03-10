use lox_parser::ast::{
    expr::{self, BinaryExpr, BinaryOp, UnaryExpr, UnaryOp, Value},
    visit::{walk_expr, Visitor},
};

use crate::error::{IResult, RuntimeError};

macro_rules! get_number {
    ($value: expr, $span: expr) => {
        match $value {
            Value::Number(n) => n,
            v => return Err(RuntimeError::type_error($span, "number", &v)),
        }
    };
}

pub struct Interpreter {}

impl Visitor for Interpreter {
    type Result = IResult<Value>;

    fn visit_literal(&mut self, literal: &expr::Value) -> Self::Result {
        Ok(literal.clone().into())
    }

    fn visit_binary(&mut self, binary: &BinaryExpr) -> Self::Result {
        let left = walk_expr(self, &binary.left)?;
        let right = walk_expr(self, &binary.right)?;

        Ok(match binary.operator {
            BinaryOp::Plus => match (left, right) {
                (Value::Number(n1), Value::Number(n2)) => (n1 + n2).into(),
                (Value::String(s1), v2) => (s1 + &v2.to_string()).into(),
                (v1, Value::String(s2)) => (v1.to_string() + &s2).into(),
                (v, Value::Number(_)) => {
                    return Err(RuntimeError::type_error(&binary.left.span, "number", &v))
                }
                (Value::Number(_), v) => {
                    return Err(RuntimeError::type_error(
                        &binary.right.span,
                        "number or string",
                        &v,
                    ))
                }
                (v, _) => {
                    return Err(RuntimeError::type_error(
                        &binary.left.span,
                        "number or string",
                        &v,
                    ))
                }
            },
            BinaryOp::Minus => {
                (get_number!(left, &binary.left.span) - get_number!(right, &binary.right.span)).into()
            }
            BinaryOp::Multiply => {
                (get_number!(left, &binary.left.span) * get_number!(right, &binary.right.span)).into()
            }
            BinaryOp::Divide => {
                (get_number!(left, &binary.left.span) / get_number!(right, &binary.right.span)).into()
            }
            BinaryOp::Equal => (left == right).into(),
            BinaryOp::NotEqual => (left != right).into(),
            _ => todo!(),
        })
    }

    fn visit_unary(&mut self, unary: &UnaryExpr) -> Self::Result {
        let operand = walk_expr(self, &unary.operand)?;
        Ok(match unary.operator {
            UnaryOp::Negative => (-get_number!(operand, &unary.operand.span)).into(),
            UnaryOp::Not => (!operand.as_bool()).into(),
        })
    }
}
