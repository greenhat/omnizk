use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_ir::ir::Module;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::operation::Operation;

use crate::ValidaTargetConfig;

pub struct ValidaTarget {
    config: ValidaTargetConfig,
}

impl Target for ValidaTarget {
    fn name(&self) -> &str {
        "ValidaVM"
    }

    fn codegen_module_old(&self, _module: Module) -> Result<Vec<u8>, CodegenError> {
        unreachable!()
    }

    fn codegen(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<Vec<u8>, CodegenError> {
        todo!()
    }
}

impl ValidaTarget {
    pub fn new(config: ValidaTargetConfig) -> ValidaTarget {
        ValidaTarget { config }
    }
}
