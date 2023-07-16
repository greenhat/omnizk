use ozk_wasm_dialect::ops as wasm;
use ozk_wasm_dialect::ops::LocalSetOp;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;
use pliron::with_context::AttachContext;

#[derive(Default)]
pub struct WasmExplicitFuncArgsPass;

impl Pass for WasmExplicitFuncArgsPass {
    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<WasmExplicitFuncArgs>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct WasmExplicitFuncArgs;

impl RewritePattern for WasmExplicitFuncArgs {
    fn match_op(&self, ctx: &Context, op: Ptr<Operation>) -> Result<bool, anyhow::Error> {
        Ok(op
            .deref(ctx)
            .get_op(ctx)
            .downcast_ref::<wasm::FuncOp>()
            .is_some())
    }

    #[allow(clippy::panic)]
    fn rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        _rewriter: &mut dyn PatternRewriter,
    ) -> Result<(), anyhow::Error> {
        let Ok(func_op) = op
            .deref(ctx)
            .get_op(ctx)
            .downcast::<wasm::FuncOp>() else {
            panic!("unexpected op {}", op.deref(ctx).with_ctx(ctx));
        };
        let func_type = func_op.get_type_typed(ctx);
        for (idx, _) in func_type.get_inputs().iter().enumerate().rev() {
            let local_set_op = LocalSetOp::new_unlinked(ctx, idx as u32).get_operation();
            local_set_op.insert_at_front(func_op.get_entry_block(ctx), ctx);
        }
        Ok(())
    }
}
