use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_ir::ir::Module;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::operation::Operation;

use crate::MidenTargetConfig;

pub struct MidenTarget {
    config: MidenTargetConfig,
}

impl Target for MidenTarget {
    fn name(&self) -> &str {
        "MidenVM"
    }

    fn compile_module_old(&self, _module: Module) -> Result<Vec<u8>, CodegenError> {
        unreachable!()
        // let inst_buf = compile_prog(module, &self.config)
        //     .map_err(|e| CodegenError::Miden(format!("{:?}", e)))?;
        // Ok(inst_buf.pretty_print().into_bytes())
    }

    fn compile(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<Vec<u8>, CodegenError> {
        todo!()
    }
}

impl MidenTarget {
    pub fn new(config: MidenTargetConfig) -> MidenTarget {
        MidenTarget { config }
    }
}
