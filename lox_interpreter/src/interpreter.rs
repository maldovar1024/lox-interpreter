use std::{
    mem,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use lox_parser::{
    ast::{
        expr::*,
        ident::{Ident, IdentTarget},
        stmt::*,
        visit::{walk_expr, walk_stmt, Visitor},
    },
    parser::Ast,
};

use crate::{
    environment::{Env, Environment, GlobalEnvironment},
    error::{IResult, RuntimeError},
    value::{Callable, Class, Function, NativeFunction, Value},
};

pub struct Interpreter {
    env: Option<Env>,
    global_env: GlobalEnvironment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut global_env = GlobalEnvironment::default();

        global_env.define(
            "clock",
            Value::NativeFunction(Rc::new(NativeFunction {
                name: "clock",
                arity: 0,
                fun: |_, _| {
                    Ok(Value::Number(
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs_f64(),
                    ))
                },
            })),
        );

        Self {
            env: None,
            global_env,
        }
    }

    pub fn interpret(&mut self, ast: &Ast) -> IResult<Value> {
        for stmt in ast {
            self.visit_stmt(stmt)?;
        }
        Ok(Value::Nil)
    }

    fn assign_to(&mut self, target: IdentTarget, value: Value) {
        self.env
            .as_deref()
            .unwrap()
            .borrow_mut()
            .assign(target, value)
    }

    fn declare_var(&mut self, ident: &Ident, value: Value) {
        match ident.target {
            Some(target) => self.assign_to(target, value),
            None => self.global_env.define(&ident.name, value),
        }
    }

    fn set_var(&mut self, ident: &Ident, value: Value) -> IResult<()> {
        match ident.target {
            Some(target) => {
                self.assign_to(target, value);
                Ok(())
            }
            None => self.global_env.assign(&ident.name, value),
        }
    }

    fn get_var(&self, ident: &Ident) -> IResult<Value> {
        match ident.target {
            Some(target) => Ok(self.env.as_deref().unwrap().borrow().get(target)),
            None => self.global_env.get(&ident.name),
        }
    }

    fn get_number(&mut self, expr: &Expr) -> IResult<f64> {
        let value = walk_expr(self, expr)?;
        match value {
            Value::Number(n) => Ok(n),
            v => Err(RuntimeError::type_error(&expr.span, "number", &v)),
        }
    }

    pub(crate) fn execute_block(
        &mut self,
        block: &[Statement],
        environment: Environment,
    ) -> IResult<Value> {
        let prev = mem::replace(&mut self.env, Some(Rc::new(environment.into())));

        let result = (|| -> IResult<Value> {
            for stmt in block.iter() {
                walk_stmt(self, stmt)?;
            }
            Ok(Value::Nil)
        })();
        self.env = prev;
        result
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for Interpreter {
    type Result = IResult<Value>;

    fn visit_print(&mut self, print: &Print) -> Self::Result {
        println!("{}", walk_expr(self, &print.expr)?);
        Ok(Value::Nil)
    }

    fn visit_block(&mut self, block: &Block) -> Self::Result {
        self.execute_block(
            &block.statements,
            Environment::new(block.num_of_locals, self.env.clone()),
        )
    }

    fn visit_if(&mut self, if_stmt: &If) -> Self::Result {
        if walk_expr(self, &if_stmt.condition)?.as_bool() {
            walk_stmt(self, &if_stmt.then_branch)?;
        } else if let Some(else_branch) = &if_stmt.else_branch {
            walk_stmt(self, else_branch)?;
        }

        Ok(Value::Nil)
    }

    fn visit_while(&mut self, while_stmt: &While) -> Self::Result {
        while walk_expr(self, &while_stmt.condition)?.as_bool() {
            walk_stmt(self, &while_stmt.body)?;
        }
        Ok(Value::Nil)
    }

    fn visit_function(&mut self, function: &FnDecl) -> Self::Result {
        //! cyclic ref here
        self.declare_var(
            &function.ident,
            Value::Function(Rc::new(Function {
                declaration: function.to_owned(),
                closure: self.env.clone(),
            })),
        );
        Ok(Value::Nil)
    }

    fn visit_class(&mut self, class: &ClassDecl) -> Self::Result {
        self.declare_var(
            &class.ident,
            Value::Class(Rc::new(Class::new(class.clone(), self.env.clone()))),
        );
        Ok(Value::Nil)
    }

    fn visit_return(&mut self, return_stmt: &Return) -> Self::Result {
        let value = match &return_stmt.expr {
            Some(expr) => walk_expr(self, expr)?,
            None => Value::Nil,
        };

        Err(RuntimeError::Return(return_stmt.span.clone(), value).to_box())
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) -> Self::Result {
        let callee = walk_expr(self, &fn_call.callee)?;
        let mut arguments = Vec::with_capacity(fn_call.arguments.len());
        for arg in fn_call.arguments.iter() {
            arguments.push(walk_expr(self, arg)?);
        }

        let f: &dyn Callable = match callee {
            Value::NativeFunction(ref f) => f.as_ref(),
            Value::Function(ref f) => f.as_ref(),
            Value::Class(ref class) => class,
            Value::Method(ref method) => method,
            _ => {
                return Err(RuntimeError::NotCallable {
                    target: callee.to_string(),
                    span: fn_call.callee.span.clone(),
                }
                .to_box())
            }
        };

        if arguments.len() != f.arity() as usize {
            return Err(RuntimeError::ArgumentsNotMatch {
                expected: f.arity(),
                got: arguments.len(),
                span: fn_call.callee.span.clone(),
            }
            .to_box());
        }

        match f.call(self, arguments) {
            Err(err) => match *err {
                RuntimeError::Return(_, v) => Ok(v),
                v => Err(v.to_box()),
            },
            v => v,
        }
    }

    fn visit_get(&mut self, get: &Get) -> Self::Result {
        let object = walk_expr(self, &get.object)?;
        if let Value::Instance(instance) = object {
            instance.borrow().get(&get.field)
        } else {
            Err(Box::new(RuntimeError::InvalidFieldTarget {
                target_type: object.type_name(),
                field: get.field.to_string(),
            }))
        }
    }

    fn visit_set(&mut self, Set { target, value }: &Set) -> Self::Result {
        let object = walk_expr(self, &target.object)?;
        if let Value::Instance(instance) = object {
            let value = walk_expr(self, value)?;
            instance.borrow_mut().set(target.field.to_string(), value.clone());
            Ok(value)
        } else {
            Err(Box::new(RuntimeError::InvalidFieldTarget {
                target_type: object.type_name(),
                field: target.field.to_string(),
            }))
        }
    }

    fn visit_assign(&mut self, assign: &Assign) -> Self::Result {
        let value = walk_expr(self, &assign.value)?;
        self.set_var(&assign.ident, value.clone())?;
        Ok(value)
    }

    fn visit_literal(&mut self, literal: &Lit) -> Self::Result {
        Ok(literal.clone().into())
    }

    fn visit_binary(&mut self, binary: &BinaryExpr) -> Self::Result {
        let BinaryExpr {
            operator,
            left,
            right,
        } = binary;

        Ok(match operator {
            BinaryOp::Plus => {
                let left = walk_expr(self, left)?;
                let right = walk_expr(self, right)?;

                match (left, right) {
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
                }
            }
            BinaryOp::Minus => (self.get_number(left)? - self.get_number(right)?).into(),
            BinaryOp::Multiply => (self.get_number(left)? * self.get_number(right)?).into(),
            BinaryOp::Divide => (self.get_number(left)? / self.get_number(right)?).into(),
            BinaryOp::Equal => (walk_expr(self, left)? == walk_expr(self, right)?).into(),
            BinaryOp::NotEqual => (walk_expr(self, left)? != walk_expr(self, right)?).into(),
            BinaryOp::Greater => (self.get_number(left)? > self.get_number(right)?).into(),
            BinaryOp::GreaterEqual => (self.get_number(left)? >= self.get_number(right)?).into(),
            BinaryOp::Less => (self.get_number(left)? < self.get_number(right)?).into(),
            BinaryOp::LessEqual => (self.get_number(left)? <= self.get_number(right)?).into(),
            BinaryOp::And | BinaryOp::Or => {
                let left = walk_expr(self, &binary.left)?;
                match binary.operator {
                    BinaryOp::And if !left.as_bool() => left,
                    BinaryOp::Or if left.as_bool() => left,
                    _ => walk_expr(self, &binary.right)?,
                }
            }
        })
    }

    fn visit_unary(&mut self, unary: &UnaryExpr) -> Self::Result {
        Ok(match unary.operator {
            UnaryOp::Negative => (-self.get_number(&unary.operand)?).into(),
            UnaryOp::Not => (!walk_expr(self, &unary.operand)?.as_bool()).into(),
        })
    }

    fn visit_ternary(&mut self, ternary: &Ternary) -> Self::Result {
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
        self.declare_var(&var_decl.ident, init);
        Ok(Value::Nil)
    }

    fn visit_var(&mut self, var: &Ident) -> Self::Result {
        self.get_var(var)
    }
}
