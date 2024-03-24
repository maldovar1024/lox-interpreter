use super::{expr::*, ident::Ident, stmt::*};

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

    fn visit_function(&mut self, function: &FnDecl) -> Self::Result;

    fn visit_class(&mut self, class: &ClassDecl) -> Self::Result;

    fn visit_return(&mut self, return_stmt: &Return) -> Self::Result;

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

    fn visit_assign(&mut self, assign: &Assign) -> Self::Result {
        walk_var(self, &assign.ident);
        walk_expr(self, &assign.value)
    }

    fn visit_group(&mut self, group: &Group) -> Self::Result {
        walk_group(self, group)
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) -> Self::Result;

    fn visit_get(&mut self, get: &Get) -> Self::Result {
        walk_expr(self, &get.object)
    }

    fn visit_literal(&mut self, literal: &Lit) -> Self::Result;

    fn visit_var(&mut self, var: &Ident) -> Self::Result;
}

pub fn walk_stmt<V: Visitor>(visitor: &mut V, stmt: &Statement) -> V::Result {
    stmt.walk(visitor)
}

pub fn walk_print<V: Visitor>(visitor: &mut V, print: &Print) -> V::Result {
    visitor.visit_expr(&print.expr)
}

pub fn walk_expression<V: Visitor>(visitor: &mut V, expression: &Expression) -> V::Result {
    visitor.visit_expr(&expression.expr)
}

pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: &Expr) -> V::Result {
    expr.expr.walk(visitor)
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

pub fn walk_var<V: Visitor>(visitor: &mut V, var: &Ident) -> V::Result {
    visitor.visit_var(var)
}
