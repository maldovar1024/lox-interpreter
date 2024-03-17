use super::expr::Expr;
use crate::{ast::visit::Visitor, ast_enum, span::Span};

#[derive(Debug, Clone)]
pub struct Print {
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub ident: String,
    pub initializer: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Box<[Statement]>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub params: Box<[String]>,
    pub body: Box<[Statement]>,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub span: Span,
    pub expr: Option<Expr>,
}

ast_enum! {
    pub enum Statement {
        visit_print: Print(Print),
        visit_expression: Expression(Expression),
        visit_var_decl: Var(VarDecl),
        visit_block: Block(Block),
        visit_if: If(If),
        visit_while: While(While),
        visit_function: FnDecl(FnDecl),
        visit_return: Return(Return),
    }
}
