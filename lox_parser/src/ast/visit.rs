use super::{
    expr::{BinaryExpr, Expr, ExprInner, FnCall, Group, Ternary, UnaryExpr, Value},
    stmt::{Block, Expression, If, Print, Statement, VarDecl, While},
};

pub trait Visitor: Sized {
    type Result;

    fn visit_stmt(&mut self, stmt: &Statement) -> Self::Result {
        walk_stmt(self, stmt)
    }

    fn visit_print(&mut self, print: &Print) -> Self::Result {
        walk_print(self, print)
    }

    fn visit_expression(&mut self, expression: &Expression) -> Self::Result {
        walk_expression(self, expression)
    }

    fn visit_if(&mut self, if_stmt: &If) -> Self::Result;

    fn visit_while(&mut self, while_stmt: &While) -> Self::Result;

    fn visit_block(&mut self, block: &Block) -> Self::Result;

    fn visit_var_decl(&mut self, var_decl: &VarDecl) -> Self::Result;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Result {
        walk_expr(self, expr)
    }

    fn visit_binary(&mut self, binary: &BinaryExpr) -> Self::Result {
        walk_binary(self, binary)
    }

    fn visit_unary(&mut self, unary: &UnaryExpr) -> Self::Result {
        walk_unary(self, unary)
    }
    fn visit_ternary(&mut self, ternary: &Ternary) -> Self::Result {
        walk_ternary(self, ternary)
    }

    fn visit_group(&mut self, group: &Group) -> Self::Result {
        walk_group(self, group)
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) -> Self::Result;

    fn visit_literal(&mut self, literal: &Value) -> Self::Result;

    fn visit_var(&mut self, var: &String) -> Self::Result;
}

pub fn walk_stmt<V: Visitor>(visitor: &mut V, stmt: &Statement) -> V::Result {
    match stmt {
        Statement::Print(p) => visitor.visit_print(p),
        Statement::Expression(e) => visitor.visit_expression(e),
        Statement::Var(var_decl) => visitor.visit_var_decl(&var_decl),
        Statement::Block(block) => visitor.visit_block(block),
        Statement::If(if_stmt) => visitor.visit_if(if_stmt),
        Statement::While(while_stmt) => visitor.visit_while(while_stmt),
    }
}

pub fn walk_print<V: Visitor>(visitor: &mut V, print: &Print) -> V::Result {
    visitor.visit_expr(&print.expr)
}

pub fn walk_expression<V: Visitor>(visitor: &mut V, expression: &Expression) -> V::Result {
    visitor.visit_expr(&expression.expr)
}

pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: &Expr) -> V::Result {
    match &expr.expr {
        ExprInner::Binary(binary) => visitor.visit_binary(binary),
        ExprInner::Unary(unary) => visitor.visit_unary(unary),
        ExprInner::Group(group) => visitor.visit_group(group),
        ExprInner::Literal(value) => visitor.visit_literal(value),
        ExprInner::Ternary(ternary) => visitor.visit_ternary(ternary),
        ExprInner::Var(var) => visitor.visit_var(var),
        ExprInner::FnCall(fn_call) => visitor.visit_fn_call(fn_call),
    }
}

pub fn walk_binary<V: Visitor>(visitor: &mut V, binary: &BinaryExpr) -> V::Result {
    visitor.visit_expr(&binary.left);
    visitor.visit_expr(&binary.right)
}

pub fn walk_unary<V: Visitor>(visitor: &mut V, unary: &UnaryExpr) -> V::Result {
    visitor.visit_expr(&unary.operand)
}

pub fn walk_ternary<V: Visitor>(visitor: &mut V, ternary: &Ternary) -> V::Result {
    visitor.visit_expr(&ternary.condition);
    visitor.visit_expr(&ternary.truthy);
    visitor.visit_expr(&ternary.falsy)
}

pub fn walk_group<V: Visitor>(visitor: &mut V, group: &Group) -> V::Result {
    visitor.visit_expr(&group.expr)
}
