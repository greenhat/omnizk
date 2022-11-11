use c2zk_codegen_tritonvm::emit;
use c2zk_ir::ir::Module;

use crate::CodegenError;
use crate::TargetConfig;

pub fn codegen(module: &Module, target_config: TargetConfig) -> Result<Vec<u8>, CodegenError> {
    Ok(match target_config {
        TargetConfig::Triton(config) => emit(module, config)?,
    })
}
