use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_codegen_tritonvm::TritonTarget;
use c2zk_ir::ir::Module;
use ozk_codegen_midenvm::MidenTarget;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::operation::Operation;

use crate::TargetConfig;

pub fn codegen(
    ctx: &mut Context,
    op: Ptr<Operation>,
    target_config: TargetConfig,
) -> Result<Vec<u8>, CodegenError> {
    Ok(match target_config {
        TargetConfig::Triton(_config) => {
            todo!()
        }
        TargetConfig::Miden(config) => {
            let target = MidenTarget::new(config);
            target.codegen(ctx, op)?
        }
    })
}

pub fn codegen_old(module: Module, target_config: TargetConfig) -> Result<Vec<u8>, CodegenError> {
    Ok(match target_config {
        TargetConfig::Triton(config) => {
            let target = TritonTarget::new(config);
            target.codegen_module_old(module)?
        }
        TargetConfig::Miden(_) => todo!(),
    })
}
