use std::{collections::HashMap, mem};

use lox_parser::{
    ast::{
        expr::*,
        ident::{IdentIndex, IdentTarget, Variable},
        stmt::*,
        visit_mut::{walk_expr, walk_stmt, VisitorMut},
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

struct VarInfo {
    index: IdentIndex,
    defined_at: Span,
    status: VariableStatus,
}

#[derive(Default)]
struct Scope {
    variables: HashMap<String, VarInfo>,
}

impl Scope {
    fn declare(&mut self, name: &str, span: Span, initialized: bool) -> Result<IdentIndex, Span> {
        match self.variables.get(name) {
            Some(var) => Err(var.defined_at),
            None => {
                let index = self.variables.len() as IdentIndex;
                self.variables.insert(
                    name.to_string(),
                    VarInfo {
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
enum ClassType {
    #[default]
    None,
    Class,
    SubClass,
}

#[derive(Default)]
enum FunctionType {
    #[default]
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Default)]
pub struct Resolver {
    scopes: Vec<Scope>,
    errors: Vec<ResolverError>,
    class_type: ClassType,
    function_type: FunctionType,
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

    fn declare(&mut self, var: &mut Variable, initialized: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            match scope.declare(&var.ident.name, var.ident.span, initialized) {
                Ok(index) => {
                    var.target = Some(IdentTarget {
                        scope_count: 0,
                        index,
                    })
                }
                Err(defined_at) => self.errors.push(ResolverError::RedefineVar {
                    pos: var.ident.span,
                    name: var.ident.name.to_string(),
                    defined_at,
                }),
            }
        }
    }

    fn access(&mut self, var: &mut Variable, status: VariableStatus) {
        for (scope_count, scope) in self.scopes.iter_mut().rev().enumerate() {
            if let Some(index) = scope.access(&var.ident.name, status) {
                var.target = Some(IdentTarget {
                    scope_count: scope_count as u16,
                    index,
                });
                break;
            }
        }
    }

    fn assign(&mut self, var: &mut Variable) {
        self.access(var, VariableStatus::Initialized);
    }

    fn get(&mut self, var: &mut Variable) {
        self.access(var, VariableStatus::Used);
    }

    fn start_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn start_class_scope(&mut self, span: Span, is_super_class: bool) {
        let mut scope = Scope::default();
        let _ = scope.declare(if is_super_class { "super" } else { "this" }, span, true);
        self.scopes.push(scope);
    }

    fn end_scope(&mut self) -> IdentIndex {
        self.scopes.pop().unwrap().variables.len() as IdentIndex
    }

    fn resolve_function(&mut self, function: &mut FnDecl) {
        self.start_scope();
        for param in function.params.iter_mut() {
            self.declare(param, true);
        }
        for stmt in function.body.iter_mut() {
            walk_stmt(self, stmt);
        }
        function.num_of_locals = self.end_scope();
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
        self.declare(&mut var_decl.var, false);
        if let Some(expr) = &mut var_decl.initializer {
            walk_expr(self, expr);
            self.assign(&mut var_decl.var);
        }
    }

    fn visit_function(&mut self, function: &mut FnDecl) -> Self::Result {
        self.declare(&mut function.var, true);
        let previous = mem::replace(&mut self.function_type, FunctionType::Function);
        self.resolve_function(function);
        self.function_type = previous;
    }

    fn visit_class(&mut self, class: &mut ClassDecl) -> Self::Result {
        self.declare(&mut class.var, true);
        let previous_class_type = mem::replace(&mut self.class_type, ClassType::Class);
        if let Some(super_class) = &mut class.super_class {
            self.start_class_scope(super_class.ident.span, true);
            self.get(super_class);
            self.class_type = ClassType::SubClass;
        }

        self.start_class_scope(class.var.ident.span, false);
        for method in class.methods.iter_mut() {
            let previous_fn_type = mem::replace(
                &mut self.function_type,
                if method.var.ident.name == "init" {
                    FunctionType::Initializer
                } else {
                    FunctionType::Method
                },
            );
            self.resolve_function(method);
            self.function_type = previous_fn_type;
        }
        self.end_scope();

        if class.super_class.is_some() {
            self.end_scope();
        }
        self.class_type = previous_class_type;
    }

    fn visit_return(&mut self, return_stmt: &mut Return) -> Self::Result {
        if matches!(self.function_type, FunctionType::None) {
            self.errors
                .push(ResolverError::InvalidReturn(return_stmt.span));
        } else if let Some(expr) = &mut return_stmt.expr {
            if matches!(self.function_type, FunctionType::Initializer) {
                self.errors
                    .push(ResolverError::ReturnInConstructor(return_stmt.span));
            }
            walk_expr(self, expr);
        }
    }

    fn visit_fn_call(&mut self, fn_call: &mut FnCall) -> Self::Result {
        walk_expr(self, &mut fn_call.callee);
        for expr in fn_call.arguments.iter_mut() {
            walk_expr(self, expr);
        }
    }

    fn visit_literal(&mut self, _literal: &mut Literal) -> Self::Result {}

    fn visit_super(&mut self, super_expr: &mut Super) -> Self::Result {
        match self.class_type {
            ClassType::SubClass => self.get(&mut super_expr.var),
            ClassType::Class => self
                .errors
                .push(ResolverError::NotSubClass(super_expr.var.ident.span)),
            ClassType::None => self
                .errors
                .push(ResolverError::InvalidSuper(super_expr.var.ident.span)),
        }
    }

    fn visit_var(&mut self, var: &mut Variable) -> Self::Result {
        if var.ident.name == "this" && matches!(self.function_type, FunctionType::None) {
            self.errors.push(ResolverError::InvalidThis(var.ident.span));
        }
        self.get(var);
    }
}
