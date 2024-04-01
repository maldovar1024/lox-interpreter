use crate::{operation::Operation, value::Value};
use lox_ast::Literal;
use lox_lexer::Span;

pub struct Chunk {
    operations: Vec<Operation>,
    constants: Vec<Value>,
    spans: Vec<Span>,
}

impl Chunk {
    pub(crate) fn add_constant(&mut self, literal: &Literal) {
        let index = self.constants.len() as u8;
        self.constants.push(Value::from_lit(&literal.value));
        self.operations.push(Operation::Constant(index));
        self.spans.push(literal.span);
    }

    pub(crate)fn add_operation(&mut self, operation: Operation, span: Span) {
        self.operations.push(operation);
        self.spans.push(span);
    }
}
