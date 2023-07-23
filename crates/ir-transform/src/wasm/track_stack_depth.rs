use ozk_wasm_dialect::op_interfaces::StackDepthChange;
use ozk_wasm_dialect::op_interfaces::TrackedStackDepth;
use ozk_wasm_dialect::ops as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::op::op_cast;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::operation::WalkOrder;
use pliron::operation::WalkResult;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;
use pliron::with_context::AttachContext;

pub struct WasmTrackStackDepthPass {
    /// If true, reserve space for local variables on the stack
    reserve_space_for_locals: bool,
}

impl WasmTrackStackDepthPass {
    /// Create a new WasmTrackStackDepthPass that reserve space for local variables on the stack
    pub fn new_reserve_space_for_locals() -> Self {
        Self {
            reserve_space_for_locals: true,
        }
    }
}

impl Pass for WasmTrackStackDepthPass {
    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::new(WasmWriteStackDepth {
            reserve_space_for_locals: self.reserve_space_for_locals,
        }));
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

pub struct WasmWriteStackDepth {
    pub reserve_space_for_locals: bool,
}

impl RewritePattern for WasmWriteStackDepth {
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
        let mut stack_depth: i32 = if self.reserve_space_for_locals {
            // reserve space for local variables
            func_op.get_locals(ctx).len() as i32
        } else {
            0
        };
        let mut ops = Vec::new();
        func_op
            .get_operation()
            .walk(ctx, WalkOrder::PostOrder, &mut |op| {
                ops.push(op);
                WalkResult::Advance
            });
        for op in ops {
            let op_op = op.deref(ctx).get_op(ctx);
            if let Some(tracked_op) = op_cast::<dyn TrackedStackDepth>(op_op.as_ref()) {
                tracked_op.set_stack_depth(ctx, stack_depth.into());
            }
            if let Some(stack_change_op) = op_cast::<dyn StackDepthChange>(op_op.as_ref()) {
                stack_depth += stack_change_op.get_stack_depth_change(ctx);
            }
        }
        Ok(())
    }
}
