use super::expr::{BinaryExpr, Expr, ExprInner, Group, UnaryExpr, Value};

pub trait Visitor: Sized {
    type Result;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Result {
        walk_expr(self, expr)
    }

    fn visit_binary(&mut self, binary: &BinaryExpr) -> Self::Result {
        walk_binary(self, binary)
    }

    fn visit_unary(&mut self, unary: &UnaryExpr) -> Self::Result {
        walk_unary(self, unary)
    }

    fn visit_group(&mut self, group: &Group) -> Self::Result {
        walk_group(self, group)
    }

    fn visit_literal(&mut self, literal: &Value) -> Self::Result;
}

pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: &Expr) -> V::Result {
    match &expr.expr {
        ExprInner::Binary(binary) => visitor.visit_binary(binary),
        ExprInner::Unary(unary) => visitor.visit_unary(unary),
        ExprInner::Group(group) => visitor.visit_group(group),
        ExprInner::Literal(value) => visitor.visit_literal(value),
    }
}

pub fn walk_binary<V: Visitor>(visitor: &mut V, binary: &BinaryExpr) -> V::Result {
    visitor.visit_expr(&binary.left);
    visitor.visit_expr(&binary.right)
}

pub fn walk_unary<V: Visitor>(visitor: &mut V, unary: &UnaryExpr) -> V::Result {
    visitor.visit_expr(&unary.operand)
}

pub fn walk_group<V: Visitor>(visitor: &mut V, group: &Group) -> V::Result {
    visitor.visit_expr(&group.expr)
}
