use ozk_wasm_dialect::op_interfaces::StackDepthChange;
use ozk_wasm_dialect::op_interfaces::TrackedStackDepth;
use ozk_wasm_dialect::ops as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::op::op_cast;
use pliron::operation::Operation;
use pliron::operation::WalkOrder;
use pliron::operation::WalkResult;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;
use pliron::with_context::AttachContext;

#[derive(Default)]
pub struct WasmTrackStackDepthPass;

impl Pass for WasmTrackStackDepthPass {
    fn name(&self) -> &str {
        "WasmTrackStackDepthPass"
    }

    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<WasmWriteStackDepth>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct WasmWriteStackDepth;

impl RewritePattern for WasmWriteStackDepth {
    fn name(&self) -> String {
        "WasmWriteStackDepth".to_string()
    }

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
        func_op: Ptr<Operation>,
        _rewriter: &mut dyn PatternRewriter,
    ) -> Result<(), anyhow::Error> {
        let Ok(_) = func_op
            .deref(ctx)
            .get_op(ctx)
            .downcast::<wasm::FuncOp>() else {
            panic!("unexpected op {}", func_op.deref(ctx).with_ctx(ctx));
        };

        let mut stack_depth: i32 = 0;
        let mut ops = Vec::new();
        func_op.walk(ctx, WalkOrder::PostOrder, &mut |op| {
            ops.push(op);
            WalkResult::Advance
        });
        for op in ops {
            let op_op = op.deref(ctx).get_op(ctx);
            if let Some(tracked_op) = op_cast::<dyn TrackedStackDepth>(op_op.as_ref()) {
                tracked_op.set_stack_depth(ctx, stack_depth as u32);
            }
            if let Some(stack_change_op) = op_cast::<dyn StackDepthChange>(op_op.as_ref()) {
                stack_depth += stack_change_op.get_stack_depth_change(ctx);
            }
        }
        Ok(())
    }
}
