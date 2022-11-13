use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_ir::ir::Module;

use crate::emit;
use crate::TritonTargetConfig;

pub struct TritonTarget {
    config: TritonTargetConfig,
}

impl Target for TritonTarget {
    fn name(&self) -> &str {
        "TritonVM"
    }

    fn compile_module(&self, module: &Module) -> Result<Vec<u8>, CodegenError> {
        let mut code = Vec::new();
        for func in module.functions() {
            for ins in func.inst() {
                let bytes = emit(ins, &self.config)?;
                code.extend(bytes);
            }
        }
        Ok(code)
    }
}

impl TritonTarget {
    pub fn new(config: TritonTargetConfig) -> TritonTarget {
        TritonTarget { config }
    }
}
