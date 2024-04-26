use thiserror::Error;

use crate::codec::DecoderError;

#[derive(Debug, Error)]
pub enum RuntimeError {}

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("{0}")]
    RuntimeError(RuntimeError),
    #[error("{0}")]
    DecoderError(DecoderError),
}

impl From<RuntimeError> for ExecutorError {
    #[inline(always)]
    fn from(value: RuntimeError) -> Self {
        Self::RuntimeError(value)
    }
}

impl From<DecoderError> for ExecutorError {
    #[inline(always)]
    fn from(value: DecoderError) -> Self {
        Self::DecoderError(value)
    }
}

pub type ExecutorResult<E> = Result<(), E>;
