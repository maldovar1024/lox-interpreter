use crate::{
    ast_enum,
    expr::Expr,
    ident::{IdentIndex, Variable},
    visit::Visitor,
    visit_mut::VisitorMut,
};
use lox_lexer::Span;

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
    pub var: Variable,
    pub initializer: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Box<[Statement]>,
    pub num_of_locals: IdentIndex,
}

impl Block {
    pub fn new(statements: Box<[Statement]>) -> Self {
        Self {
            statements,
            num_of_locals: 0,
        }
    }
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
    pub var: Variable,
    pub params: Box<[Variable]>,
    pub body: Box<[Statement]>,
    pub num_of_locals: IdentIndex,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub span: Span,
    pub expr: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub var: Variable,
    pub super_class: Option<Variable>,
    pub methods: Box<[FnDecl]>,
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
        visit_class: ClassDecl(ClassDecl),
    }
}
