/*
use ozk_codegen_shared::CodegenError;
use ozk_codegen_shared::Target;
use ozk_ir::ir::Module;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::operation::Operation;

use crate::compile_module;
use crate::TritonTargetConfig;

pub struct TritonTarget {
    config: TritonTargetConfig,
}

impl Target for TritonTarget {
    fn name(&self) -> &str {
        "TritonVM"
    }

    fn codegen_module_old(&self, module: Module) -> Result<Vec<u8>, CodegenError> {
        let inst_buf = compile_module(module, &self.config)
            .map_err(|e| CodegenError::Triton(format!("{:?}", e)))?;
        Ok(inst_buf.pretty_print().into_bytes())
    }

    fn codegen(&self, _ctx: &mut Context, _op: Ptr<Operation>) -> Result<Vec<u8>, CodegenError> {
        todo!()
    }
}

impl TritonTarget {
    pub fn new(config: TritonTargetConfig) -> TritonTarget {
        TritonTarget { config }
    }
}
*/
