pub use c2zk_codegen_tritonvm::TritonTargetConfig;

pub enum TargetConfig {
    Triton(TritonTargetConfig),
}
