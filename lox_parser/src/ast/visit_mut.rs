use super::{expr::*, ident::Ident, stmt::*};

pub trait VisitorMut: Sized {
    type Result;

    fn visit_stmt(&mut self, stmt: &mut Statement) -> Self::Result {
        walk_stmt(self, stmt)
    }

    fn visit_print(&mut self, print: &mut Print) -> Self::Result {
        walk_print(self, print)
    }

    fn visit_expression(&mut self, expression: &mut Expression) -> Self::Result {
        walk_expression(self, expression)
    }

    fn visit_if(&mut self, if_stmt: &mut If) -> Self::Result;

    fn visit_while(&mut self, while_stmt: &mut While) -> Self::Result;

    fn visit_block(&mut self, block: &mut Block) -> Self::Result;

    fn visit_var_decl(&mut self, var_decl: &mut VarDecl) -> Self::Result;

    fn visit_function(&mut self, function: &mut FnDecl) -> Self::Result;

    fn visit_class(&mut self, class: &mut ClassDecl) -> Self::Result;
  
    fn visit_return(&mut self, return_stmt: &mut Return) -> Self::Result;

    fn visit_expr(&mut self, expr: &mut Expr) -> Self::Result {
        walk_expr(self, expr)
    }

    fn visit_binary(&mut self, binary: &mut BinaryExpr) -> Self::Result {
        walk_binary(self, binary)
    }

    fn visit_unary(&mut self, unary: &mut UnaryExpr) -> Self::Result {
        walk_unary(self, unary)
    }
    fn visit_ternary(&mut self, ternary: &mut Ternary) -> Self::Result {
        walk_ternary(self, ternary)
    }

    fn visit_group(&mut self, group: &mut Group) -> Self::Result {
        walk_group(self, group)
    }

    fn visit_fn_call(&mut self, fn_call: &mut FnCall) -> Self::Result;

    fn visit_literal(&mut self, literal: &mut Lit) -> Self::Result;

    fn visit_var(&mut self, var: &mut Ident) -> Self::Result;
}

pub fn walk_stmt<V: VisitorMut>(visitor: &mut V, stmt: &mut Statement) -> V::Result {
    stmt.walk_mut(visitor)
}

pub fn walk_print<V: VisitorMut>(visitor: &mut V, print: &mut Print) -> V::Result {
    visitor.visit_expr(&mut print.expr)
}

pub fn walk_expression<V: VisitorMut>(visitor: &mut V, expression: &mut Expression) -> V::Result {
    visitor.visit_expr(&mut expression.expr)
}

pub fn walk_expr<V: VisitorMut>(visitor: &mut V, expr: &mut Expr) -> V::Result {
    expr.expr.walk_mut(visitor)
}

pub fn walk_binary<V: VisitorMut>(visitor: &mut V, binary: &mut BinaryExpr) -> V::Result {
    visitor.visit_expr(&mut binary.left);
    visitor.visit_expr(&mut binary.right)
}

pub fn walk_unary<V: VisitorMut>(visitor: &mut V, unary: &mut UnaryExpr) -> V::Result {
    visitor.visit_expr(&mut unary.operand)
}

pub fn walk_ternary<V: VisitorMut>(visitor: &mut V, ternary: &mut Ternary) -> V::Result {
    visitor.visit_expr(&mut ternary.condition);
    visitor.visit_expr(&mut ternary.truthy);
    visitor.visit_expr(&mut ternary.falsy)
}

pub fn walk_group<V: VisitorMut>(visitor: &mut V, group: &mut Group) -> V::Result {
    visitor.visit_expr(&mut group.expr)
}
