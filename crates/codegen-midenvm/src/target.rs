use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_ir::ir::Module;
use ozk_miden_dialect::ops::ProgramOp;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::operation::Operation;
use pliron::with_context::AttachContext;

use crate::emit_prog;
use crate::MidenError;
use crate::MidenTargetConfig;

pub struct MidenTarget {
    config: MidenTargetConfig,
}

impl Target for MidenTarget {
    fn name(&self) -> &str {
        "MidenVM"
    }

    fn codegen_module_old(&self, _module: Module) -> Result<Vec<u8>, CodegenError> {
        unreachable!()
        // let inst_buf = compile_prog(module, &self.config)
        //     .map_err(|e| CodegenError::Miden(format!("{:?}", e)))?;
        // Ok(inst_buf.pretty_print().into_bytes())
    }

    fn codegen(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<Vec<u8>, CodegenError> {
        if let Some(prog_op) = op.deref(ctx).get_op(ctx).downcast_ref::<ProgramOp>() {
            let inst_buf =
                emit_prog(ctx, prog_op, &self.config).map_err(|e| CodegenError::Miden(e.into()))?;
            Ok(inst_buf.pretty_print().into_bytes())
        } else {
            Err(CodegenError::Miden(
                MidenError::InvalidInst(format!(
                    "expected ProgramOp, got {:?}",
                    op.with_ctx(ctx).to_string()
                ))
                .into(),
            ))
        }
    }
}

impl MidenTarget {
    pub fn new(config: MidenTargetConfig) -> MidenTarget {
        MidenTarget { config }
    }
}
