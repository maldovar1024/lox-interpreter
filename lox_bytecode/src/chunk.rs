use crate::{operation::Operation, string::StringIntern};
use lox_ast::{Lit, Literal};
use lox_lexer::Span;

#[derive(Debug, Default)]
pub struct Chunk {
    operations: Vec<Operation>,
    spans: Vec<Span>,
    strings: StringIntern,
}

impl Chunk {
    pub fn iter(&self) -> impl Iterator<Item = &Operation> {
        self.operations.iter()
    }

    pub fn get_span_at(&self, index: usize) -> Span {
        self.spans[index]
    }

    pub(crate) fn add_constant(&mut self, literal: &Literal) {
        let operation = match &literal.value {
            Lit::Number(n) => Operation::LoadNumber(*n),
            Lit::String(s) => Operation::LoadString(self.strings.intern(s)),
            Lit::Bool(b) => Operation::LoadBool(*b),
            Lit::Nil => Operation::LoadNil,
        };
        self.add_operation(operation, literal.span);
    }

    pub(crate) fn add_operation(&mut self, operation: Operation, span: Span) {
        self.operations.push(operation);
        self.spans.push(span);
    }
}
