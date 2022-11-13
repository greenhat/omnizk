use c2zk_codegen_shared::CodegenError;
use c2zk_frontend::FrontendError;
use derive_more::From;

#[derive(Debug, From)]
pub enum CompileError {
    FrontendError(FrontendError),
    CodegenError(CodegenError),
}
