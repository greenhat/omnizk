use anyhow::anyhow;
use ozk_miden_dialect as miden;
use ozk_wasm_dialect as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use wasm::types::i32_type;

#[derive(Default)]
pub struct ArithOpLowering {}

impl RewritePattern for ArithOpLowering {
    fn match_op(&self, ctx: &Context, op: Ptr<Operation>) -> Result<bool, anyhow::Error> {
        Ok(op
            .deref(ctx)
            .get_op(ctx)
            .downcast_ref::<wasm::ops::AddOp>()
            .is_some())
    }

    #[allow(clippy::unwrap_used)]
    fn rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        rewriter: &mut dyn PatternRewriter,
    ) -> Result<(), anyhow::Error> {
        let opop = &op.deref(ctx).get_op(ctx);
        if let Some(add_op) = opop.downcast_ref::<wasm::ops::AddOp>() {
            let add_op_ty = add_op.get_type(ctx);
            if add_op_ty == i32_type(ctx) {
                let miden_op = miden::ops::AddOp::new_unlinked(ctx);
                rewriter.replace_op_with(ctx, op, miden_op.get_operation())?;
            } else {
                return Err(anyhow!("only 32-bit integers are supported"));
            }
        }
        Ok(())
    }
}
