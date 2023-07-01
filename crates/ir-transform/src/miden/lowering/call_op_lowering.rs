use ozk_miden_dialect::ops as miden;
use ozk_wasm_dialect::ops as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::operation::WalkOrder;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;

#[derive(Default)]
pub struct WasmToMidenCallOpLoweringPass;

impl Pass for WasmToMidenCallOpLoweringPass {
    fn name(&self) -> &str {
        "WasmToMidenCallOpLoweringPass"
    }

    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<CallOpLowering>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct CallOpLowering;

impl RewritePattern for CallOpLowering {
    fn name(&self) -> String {
        "CallOpLowering".to_string()
    }

    fn match_op(&self, ctx: &Context, op: Ptr<Operation>) -> Result<bool, anyhow::Error> {
        Ok(op
            .deref(ctx)
            .get_op(ctx)
            .downcast_ref::<wasm::ModuleOp>()
            .is_some())
    }

    fn rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        rewriter: &mut dyn PatternRewriter,
    ) -> Result<(), anyhow::Error> {
        let Ok(module_op) = op
            .deref(ctx)
            .get_op(ctx)
            .downcast::<wasm::ModuleOp>() else {
            todo!("error");
        };
        let mut call_ops = Vec::new();
        module_op
            .get_operation()
            .walk(ctx, WalkOrder::PostOrder, &mut |op| {
                if let Ok(call_op) = op.deref(ctx).get_op(ctx).downcast::<wasm::CallOp>() {
                    call_ops.push(call_op);
                }
                pliron::operation::WalkResut::Advance
            });
        for call_op in call_ops {
            let callee_sym = module_op.get_func_sym(ctx, call_op.get_func_index(ctx));
            let miden_exec_op = miden::ExecOp::new_unlinked(ctx, callee_sym);
            rewriter.replace_op_with(
                ctx,
                call_op.get_operation(),
                miden_exec_op.get_operation(),
            )?;
        }
        Ok(())
    }
}
