use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_codegen_tritonvm::TritonTarget;
use c2zk_ir::ir::Module;

use crate::TargetConfig;

pub fn codegen(module: Module, target_config: TargetConfig) -> Result<Vec<u8>, CodegenError> {
    Ok(match target_config {
        TargetConfig::Triton(config) => {
            let target = TritonTarget::new(config);
            target.compile_module(module)?
        }
    })
}
