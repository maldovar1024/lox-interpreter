use super::expr::Expr;

#[derive(Debug)]
pub struct Print {
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Expression {
    pub expr: Expr,
}

#[derive(Debug)]
pub struct VarDecl {
    pub ident: String,
    pub initializer: Option<Expr>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Box<[Statement]>,
}

#[derive(Debug)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

#[derive(Debug)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Statement>,
}

#[derive(Debug)]
pub struct FnDecl {
    pub name: String,
    pub params: Box<[String]>,
    pub body: Box<[Statement]>,
}

#[derive(Debug)]
pub enum Statement {
    Print(Print),
    Expression(Expression),
    Var(VarDecl),
    Block(Block),
    If(If),
    While(While),
    FnDecl(FnDecl)
}
