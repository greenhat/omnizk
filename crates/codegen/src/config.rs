pub use c2zk_codegen_tritonvm::TritonTargetConfig;
use c2zk_ir::pass::IrPass;
use derive_more::From;
use ozk_codegen_midenvm::MidenTargetConfig;
use pliron::context::Context;
use pliron::pass::PassManager;

#[derive(From)]
pub enum TargetConfig {
    Triton(TritonTargetConfig),
    Miden(MidenTargetConfig),
}

impl TargetConfig {
    pub fn ir_passes(&self) -> &[Box<dyn IrPass>] {
        match self {
            TargetConfig::Triton(c) => c.ir_passes.as_slice(),
            TargetConfig::Miden(_) => todo!(),
        }
    }

    pub fn pass_manager(&self) -> &PassManager {
        match self {
            TargetConfig::Triton(_c) => todo!(),
            TargetConfig::Miden(m) => &m.pass_manager,
        }
    }

    pub fn register(&self, ctx: &mut Context) {
        match self {
            TargetConfig::Triton(_c) => todo!(),
            TargetConfig::Miden(m) => m.register(ctx),
        }
    }
}
