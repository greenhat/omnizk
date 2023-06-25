use anyhow::anyhow;
use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_ir::ir::Module;
use ozk_valida_dialect as valida;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::operation::Operation;
use pliron::with_context::AttachContext;

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
        if let Some(prog_op) = op
            .deref(ctx)
            .get_op(ctx)
            .downcast_ref::<valida::ops::ProgramOp>()
        {
            todo!("compile valida program");
        } else {
            Err(CodegenError::Valida(anyhow!(
                "expected builtin.module, got {:?}",
                op.with_ctx(ctx).to_string()
            )))
        }
    }
}

impl ValidaTarget {
    pub fn new(config: ValidaTargetConfig) -> ValidaTarget {
        ValidaTarget { config }
    }
}
