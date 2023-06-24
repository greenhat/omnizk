#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    #[error("Triton VM codegen error: {0}")]
    Triton(String),
    #[error("Miden VM codegen error: {0}")]
    Miden(anyhow::Error),
    #[error("Valida VM codegen error: {0}")]
    Valida(anyhow::Error),
}
