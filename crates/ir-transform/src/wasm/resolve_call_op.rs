use anyhow::Ok;
use ozk_ozk_dialect as ozk;
use ozk_wasm_dialect as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::operation::WalkOrder;
use pliron::operation::WalkResult;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;

#[derive(Default)]
pub struct WasmCallOpToOzkCallOpPass;

impl Pass for WasmCallOpToOzkCallOpPass {
    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<WasmCallOpToOzkCallOp>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct WasmCallOpToOzkCallOp;

impl RewritePattern for WasmCallOpToOzkCallOp {
    fn match_and_rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        rewriter: &mut dyn PatternRewriter,
    ) -> Result<bool, anyhow::Error> {
        let opop = &op.deref(ctx).get_op(ctx);
        let Some(module_op) = opop.downcast_ref::<wasm::ops::ModuleOp>() else {
            return Ok(false);
        };
        let mut wasm_call_ops = Vec::new();
        module_op.get_operation().walk_only::<wasm::ops::CallOp>(
            ctx,
            WalkOrder::PostOrder,
            &mut |op| {
                wasm_call_ops.push(*op);
                WalkResult::Advance
            },
        );

        for wasm_call_op in wasm_call_ops {
            #[allow(clippy::expect_used)]
            let func_sym = module_op
                .get_func_sym(ctx, wasm_call_op.get_func_index(ctx))
                .expect("func_sym not found");
            #[allow(clippy::expect_used)]
            let func_op = module_op.get_func(ctx, &func_sym).expect("func not found");
            let call_op = ozk::ops::CallOp::new_unlinked(ctx, func_sym, func_op.get_type(ctx));
            rewriter.replace_op_with(ctx, wasm_call_op.get_operation(), call_op.get_operation())?;
        }

        Ok(true)
    }
}
