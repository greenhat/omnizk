use c2zk_ir::ir::Inst;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MidenError {
    #[error("Invalid instruction: {0}")]
    InvalidInst(String),
    #[error("Unsupported instruction: {0:?}")]
    UnsupportedInstruction(Inst),
}
