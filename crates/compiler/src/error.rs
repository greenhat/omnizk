use c2zk_codegen_shared::CodegenError;
use c2zk_frontend::FrontendError;
use pliron::pass::PassError;

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    #[error("Frontend error: {0}")]
    FrontendError(#[from] FrontendError),
    #[error("Codegen error: {0}")]
    CodegenError(#[from] CodegenError),
    #[error("Pass error: {0}")]
    PassError(#[from] PassError),
}
