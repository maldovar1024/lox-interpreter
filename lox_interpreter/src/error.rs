use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("TypeError: expected `{expected}`, found `{found}")]
    TypeError {
        expected: &'static str,
        found: &'static str,
    },
}

impl RuntimeError {
    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }
}

pub type IResult<T> = Result<T, Box<RuntimeError>>;
