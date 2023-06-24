pub use c2zk_codegen_tritonvm::TritonTargetConfig;
use c2zk_ir::pass::IrPass;
use derive_more::From;
use ozk_codegen_midenvm::MidenTargetConfig;
use ozk_codegen_valida::ValidaTargetConfig;
use pliron::context::Context;
use pliron::pass::PassManager;

#[derive(From)]
pub enum TargetConfig {
    Triton(TritonTargetConfig),
    Miden(MidenTargetConfig),
    Valida(ValidaTargetConfig),
}

impl TargetConfig {
    pub fn ir_passes(&self) -> &[Box<dyn IrPass>] {
        match self {
            TargetConfig::Triton(c) => c.ir_passes.as_slice(),
            TargetConfig::Miden(_) => todo!(),
            TargetConfig::Valida(_) => todo!(),
        }
    }

    pub fn pass_manager(&self) -> &PassManager {
        match self {
            TargetConfig::Triton(_c) => todo!(),
            TargetConfig::Miden(m) => &m.pass_manager,
            TargetConfig::Valida(v) => &v.pass_manager,
        }
    }

    pub fn register(&self, ctx: &mut Context) {
        match self {
            TargetConfig::Triton(_c) => todo!(),
            TargetConfig::Miden(m) => m.register(ctx),
            TargetConfig::Valida(v) => v.register(ctx),
        }
    }
}
