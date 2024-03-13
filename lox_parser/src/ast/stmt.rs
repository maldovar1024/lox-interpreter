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
pub enum Statement {
    Print(Print),
    Expression(Expression),
    Var(VarDecl),
    Block(Block),
}
