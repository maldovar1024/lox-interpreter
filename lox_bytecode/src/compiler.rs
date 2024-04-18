use lox_ast::{
    visit::{walk_binary, walk_unary, Visitor},
    *,
};
use lox_bytecode_ops::{Operation, StringIntern};
use lox_lexer::Span;
use lox_parser::parser::Ast;

#[derive(Debug, Default)]
pub struct Compiler {
    operations: Vec<Operation>,
    spans: Vec<Span>,
    strings: StringIntern,
}

impl Compiler {
    pub fn compile(&mut self, ast: &Ast) {
        for stmt in ast {
            self.visit_stmt(stmt);
        }
    }

    pub fn get_span_at(&self, index: usize) -> Span {
        self.spans[index]
    }

    fn add_constant(&mut self, literal: &Literal) {
        let operation = match &literal.value {
            Lit::Number(n) => Operation::LoadNumber(*n),
            Lit::String(s) => Operation::LoadString(self.strings.intern(s)),
            Lit::Bool(b) => Operation::LoadBool(*b),
            Lit::Nil => Operation::LoadNil,
        };
        self.add_operation(operation, literal.span);
    }

    fn add_operation(&mut self, operation: Operation, span: Span) {
        self.operations.push(operation);
        self.spans.push(span);
    }
}

impl Visitor for Compiler {
    type Result = ();

    fn visit_if(&mut self, if_stmt: &If) -> Self::Result {
        todo!()
    }

    fn visit_while(&mut self, while_stmt: &While) -> Self::Result {
        todo!()
    }

    fn visit_block(&mut self, block: &Block) -> Self::Result {
        todo!()
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl) -> Self::Result {
        todo!()
    }

    fn visit_function(&mut self, function: &FnDecl) -> Self::Result {
        todo!()
    }

    fn visit_class(&mut self, class: &ClassDecl) -> Self::Result {
        todo!()
    }

    fn visit_return(&mut self, return_stmt: &Return) -> Self::Result {
        todo!()
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) -> Self::Result {
        todo!()
    }

    fn visit_super(&mut self, super_expr: &Super) -> Self::Result {
        todo!()
    }

    fn visit_unary(&mut self, unary: &UnaryExpr) -> Self::Result {
        walk_unary(self, unary);
        self.add_operation(unary.operator.into(), unary.get_span());
    }

    fn visit_binary(&mut self, binary: &BinaryExpr) -> Self::Result {
        walk_binary(self, binary);
        let span = binary.get_span();
        self.add_operation(binary.operator.into(), span)
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Result {
        self.add_constant(literal);
    }

    fn visit_var(&mut self, var: &Variable) -> Self::Result {
        todo!()
    }
}
