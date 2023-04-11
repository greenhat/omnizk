use thiserror::Error;

use crate::EmitError;

#[derive(Debug, Error)]
pub enum MidenError {
    #[error("Invalid instruction: {0}")]
    InvalidInst(String),
    #[error("Emit error: {0:?}")]
    Emit(#[from] EmitError),
}
