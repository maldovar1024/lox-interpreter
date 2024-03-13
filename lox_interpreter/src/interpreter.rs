use lox_parser::{
    ast::{
        expr::{self, BinaryExpr, BinaryOp, ExprInner, UnaryExpr, UnaryOp, Value},
        stmt::{Print, VarDecl},
        visit::{walk_expr, Visitor},
    },
    parser::Ast,
};

use crate::{
    environment::Environment,
    error::{IResult, RuntimeError},
};

macro_rules! get_number {
    ($value: expr, $span: expr) => {
        match $value {
            Value::Number(n) => n,
            v => return Err(RuntimeError::type_error($span, "number", &v)),
        }
    };
}

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::default(),
        }
    }

    pub fn interpret(&mut self, ast: &Ast) -> IResult<Value> {
        for stmt in ast {
            self.visit_stmt(stmt)?;
        }
        Ok(Value::Nil)
    }

    fn assign(&mut self, binary: &BinaryExpr) -> IResult<Value> {
        assert!(matches!(binary.operator, BinaryOp::Assign));
        let right = walk_expr(self, &binary.right)?;
        match &binary.left.expr {
            ExprInner::Var(var) => {
                self.env.assign(&var, right.clone())?;
                Ok(right)
            }
            _ => Err(RuntimeError::InvalidLeftValue(binary.left.span.to_owned()).to_box()),
        }
    }
}

impl Visitor for Interpreter {
    type Result = IResult<Value>;

    fn visit_print(&mut self, print: &Print) -> Self::Result {
        println!("{}", walk_expr(self, &print.expr)?);
        Ok(Value::Nil)
    }

    fn visit_literal(&mut self, literal: &expr::Value) -> Self::Result {
        Ok(literal.clone().into())
    }

    fn visit_binary(&mut self, binary: &BinaryExpr) -> Self::Result {
        if matches!(binary.operator, BinaryOp::Assign) {
            return self.assign(binary);
        }

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
            BinaryOp::Minus => (get_number!(left, &binary.left.span)
                - get_number!(right, &binary.right.span))
            .into(),
            BinaryOp::Multiply => (get_number!(left, &binary.left.span)
                * get_number!(right, &binary.right.span))
            .into(),
            BinaryOp::Divide => (get_number!(left, &binary.left.span)
                / get_number!(right, &binary.right.span))
            .into(),
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

    fn visit_ternary(&mut self, ternary: &expr::Ternary) -> Self::Result {
        let condition = walk_expr(self, &ternary.condition)?;
        if condition.as_bool() {
            walk_expr(self, &ternary.truthy)
        } else {
            walk_expr(self, &ternary.falsy)
        }
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl) -> Self::Result {
        let init = match &var_decl.initializer {
            Some(expr) => walk_expr(self, expr)?,
            None => Value::Nil,
        };
        self.env.define(&var_decl.ident, init);
        Ok(Value::Nil)
    }

    fn visit_var(&mut self, var: &String) -> Self::Result {
        self.env.get(&var)
    }
}
