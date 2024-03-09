use lox_parser::ast::{
    expr::{self, BinaryExpr, BinaryOp},
    visit::{walk_expr, Visitor},
};

use crate::{error::IResult, value::Value};

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
            BinaryOp::Plus => match left.get_number() {
                Ok(left) => (left + right.get_number()?).into(),
                _ => (left.get_string()? + &right.get_string()?).into(),
            },
            BinaryOp::Minus => (left.get_number()? - right.get_number()?).into(),
            BinaryOp::Multiply => (left.get_number()? * right.get_number()?).into(),
            BinaryOp::Divide => (left.get_number()? / right.get_number()?).into(),
            BinaryOp::Equal => (left == right).into(),
            BinaryOp::NotEqual => (left != right).into(),
            _ => todo!(),
        })
    }

    fn visit_unary(&mut self, unary: &expr::UnaryExpr) -> Self::Result {
        let operand = walk_expr(self, &unary.operand)?;
        Ok(match unary.operator {
            expr::UnaryOp::Negative => (-operand.get_number()?).into(),
            expr::UnaryOp::Not => (!operand.as_bool()).into(),
        })
    }
}
