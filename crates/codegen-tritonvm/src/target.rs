use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_ir::ir::Module;

use crate::emit;
use crate::InstBuffer;
use crate::TritonTargetConfig;

pub struct TritonTarget {
    config: TritonTargetConfig,
}

impl Target for TritonTarget {
    fn name(&self) -> &str {
        "TritonVM"
    }

    fn compile_module(&self, module: &Module) -> Result<Vec<u8>, CodegenError> {
        let mut sink = InstBuffer::new(&self.config);
        for func in module.functions() {
            for ins in func.inst() {
                emit(ins, &self.config, &mut sink)?;
            }
        }
        Ok(sink.pretty_print().into_bytes())
    }
}

impl TritonTarget {
    pub fn new(config: TritonTargetConfig) -> TritonTarget {
        TritonTarget { config }
    }
}
