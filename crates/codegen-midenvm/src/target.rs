use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_ir::ir::Module;

use crate::compile_module;
use crate::MidenTargetConfig;

pub struct MidenTarget {
    config: MidenTargetConfig,
}

impl Target for MidenTarget {
    fn name(&self) -> &str {
        "MidenVM"
    }

    fn compile_module(&self, module: Module) -> Result<Vec<u8>, CodegenError> {
        let inst_buf = compile_module(module, &self.config)
            .map_err(|e| CodegenError::Miden(format!("{:?}", e)))?;
        Ok(inst_buf.pretty_print().into_bytes())
    }
}

impl MidenTarget {
    pub fn new(config: MidenTargetConfig) -> MidenTarget {
        MidenTarget { config }
    }
}
