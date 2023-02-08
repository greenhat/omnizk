pub use c2zk_codegen_tritonvm::TritonTargetConfig;
use c2zk_ir::pass::IrPass;

pub enum TargetConfig {
    Triton(TritonTargetConfig),
}

impl TargetConfig {
    pub fn ir_passes(&self) -> &[Box<dyn IrPass>] {
        match self {
            TargetConfig::Triton(c) => c.ir_passes.as_slice(),
        }
    }
}
