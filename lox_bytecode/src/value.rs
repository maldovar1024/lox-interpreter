use lox_ast::Lit;

pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn from_lit(lit: &Lit) -> Self {
        match lit {
            Lit::Number(n) => Value::Number(*n),
            Lit::String(_) => todo!(),
            Lit::Bool(b) => Value::Bool(*b),
            Lit::Nil => Value::Nil,
        }
    }
}
