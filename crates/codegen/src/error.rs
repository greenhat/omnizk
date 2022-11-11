use c2zk_codegen_tritonvm::TritonError;
use derive_more::From;

#[derive(Debug, From)]
pub enum CodegenError {
    Triton(TritonError),
}
