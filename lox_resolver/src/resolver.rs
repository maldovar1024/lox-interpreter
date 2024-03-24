use std::{collections::HashMap, mem};

use lox_parser::{
    ast::{
        expr::*,
        ident::{Ident, IdentIndex, IdentTarget},
        stmt::*,
        visit_mut::{walk_binary, walk_expr, walk_stmt, VisitorMut},
    },
    parser::Ast,
    span::Span,
};

use crate::error::ResolverError;

#[derive(Clone, Copy)]
enum VariableStatus {
    Declared,
    Initialized,
    Used,
}

struct Variable {
    index: IdentIndex,
    defined_at: Span,
    status: VariableStatus,
}

#[derive(Default)]
struct Scope {
    variables: HashMap<String, Variable>,
}

impl Scope {
    fn declare(&mut self, name: &str, span: Span, initialized: bool) -> Result<IdentIndex, Span> {
        match self.variables.get(name) {
            Some(var) => Err(var.defined_at.clone()),
            None => {
                let index = self.variables.len() as IdentIndex;
                self.variables.insert(
                    name.to_string(),
                    Variable {
                        index,
                        defined_at: span,
                        status: if initialized {
                            VariableStatus::Initialized
                        } else {
                            VariableStatus::Declared
                        },
                    },
                );
                Ok(index)
            }
        }
    }

    fn access(&mut self, name: &str, status: VariableStatus) -> Option<IdentIndex> {
        self.variables.get_mut(name).map(|var| {
            var.status = status;
            var.index
        })
    }
}

#[derive(Default)]
pub struct Resolver {
    scopes: Vec<Scope>,
    errors: Vec<ResolverError>,
}

impl Resolver {
    pub fn resolve(&mut self, ast: &mut Ast) -> Option<Box<[ResolverError]>> {
        ast.iter_mut().for_each(|stmt| self.visit_stmt(stmt));
        if self.errors.is_empty() {
            None
        } else {
            Some(mem::take(&mut self.errors).into_boxed_slice())
        }
    }

    fn declare(&mut self, ident: &mut Ident, initialized: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            match scope.declare(&ident.name, ident.span.clone(), initialized) {
                Ok(index) => {
                    ident.target = Some(IdentTarget {
                        scope_count: 0,
                        index,
                    })
                }
                Err(defined_at) => self.errors.push(ResolverError::RedefineVar {
                    pos: ident.span.clone(),
                    name: ident.name.to_string(),
                    defined_at,
                }),
            }
        }
    }

    fn access(&mut self, ident: &mut Ident, status: VariableStatus) {
        for (scope_count, scope) in self.scopes.iter_mut().rev().enumerate() {
            if let Some(index) = scope.access(&ident.name, status) {
                ident.target = Some(IdentTarget {
                    scope_count: scope_count as u16,
                    index,
                });
                break;
            }
        }
    }

    fn assign(&mut self, ident: &mut Ident) {
        self.access(ident, VariableStatus::Initialized);
    }

    fn get(&mut self, ident: &mut Ident) {
        self.access(ident, VariableStatus::Used);
    }

    fn start_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn end_scope(&mut self) -> IdentIndex {
        self.scopes.pop().unwrap().variables.len() as IdentIndex
    }
}

impl VisitorMut for Resolver {
    type Result = ();

    fn visit_if(&mut self, if_stmt: &mut If) -> Self::Result {
        walk_expr(self, &mut if_stmt.condition);
        walk_stmt(self, &mut if_stmt.then_branch);
        if let Some(else_branch) = &mut if_stmt.else_branch {
            walk_stmt(self, else_branch);
        }
    }

    fn visit_while(&mut self, while_stmt: &mut While) -> Self::Result {
        walk_expr(self, &mut while_stmt.condition);
        walk_stmt(self, &mut while_stmt.body);
    }

    fn visit_block(&mut self, block: &mut Block) -> Self::Result {
        self.start_scope();
        for stmt in block.statements.iter_mut() {
            walk_stmt(self, stmt);
        }
        block.num_of_locals = self.end_scope();
    }

    fn visit_var_decl(&mut self, var_decl: &mut VarDecl) -> Self::Result {
        self.declare(&mut var_decl.ident, false);
        if let Some(expr) = &mut var_decl.initializer {
            walk_expr(self, expr);
            self.assign(&mut var_decl.ident);
        }
    }

    fn visit_function(&mut self, function: &mut FnDecl) -> Self::Result {
        self.declare(&mut function.ident, true);
        self.start_scope();
        for param in function.params.iter_mut() {
            self.declare(param, true);
        }
        for stmt in function.body.iter_mut() {
            walk_stmt(self, stmt);
        }
        function.num_of_locals = self.end_scope();
    }

    fn visit_return(&mut self, return_stmt: &mut Return) -> Self::Result {
        if let Some(expr) = &mut return_stmt.expr {
            walk_expr(self, expr);
        }
    }

    fn visit_fn_call(&mut self, fn_call: &mut FnCall) -> Self::Result {
        walk_expr(self, &mut fn_call.callee);
        for expr in fn_call.arguments.iter_mut() {
            walk_expr(self, expr);
        }
    }

    fn visit_literal(&mut self, _literal: &mut Lit) -> Self::Result {}

    fn visit_var(&mut self, var: &mut Ident) -> Self::Result {
        self.get(var);
    }

    fn visit_binary(&mut self, binary: &mut BinaryExpr) -> Self::Result {
        walk_binary(self, binary);
        if matches!(binary.operator, BinaryOp::Assign) {
            if let ExprInner::Var(var) = &mut binary.left.expr {
                self.get(var);
            } else {
                self.errors
                    .push(ResolverError::InvalidLeftValue(binary.left.span.clone()));
            }
        }
    }
}
