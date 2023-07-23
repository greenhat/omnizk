use thiserror::Error;

use crate::EmitError;
use crate::TopoSortError;

#[derive(Debug, Error)]
pub enum MidenError {
    #[error("Invalid instruction: {0}")]
    InvalidInst(String),
    #[error("Emit error: {0:?}")]
    Emit(#[from] EmitError),
    #[error("Topological sort error: {0:?}")]
    TopoSortError(#[from] TopoSortError),
}
