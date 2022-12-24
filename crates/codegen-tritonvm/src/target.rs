use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_ir::ir::Module;

use crate::compile_module;
use crate::TritonTargetConfig;

pub struct TritonTarget {
    config: TritonTargetConfig,
}

impl Target for TritonTarget {
    fn name(&self) -> &str {
        "TritonVM"
    }

    fn compile_module(&self, module: Module) -> Result<Vec<u8>, CodegenError> {
        let inst_buf = compile_module(module, &self.config)
            .map_err(|e| CodegenError::Triton(format!("{:?}", e)))?;
        Ok(inst_buf.pretty_print().into_bytes())
    }
}

impl TritonTarget {
    pub fn new(config: TritonTargetConfig) -> TritonTarget {
        TritonTarget { config }
    }
}
