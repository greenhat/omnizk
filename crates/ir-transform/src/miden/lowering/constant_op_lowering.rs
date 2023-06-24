use anyhow::anyhow;
use ozk_miden_dialect as miden;
use ozk_ozk_dialect::attributes::oxfoi_field_elem_from_int;
use ozk_wasm_dialect as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;

#[derive(Default)]
pub struct ConstantOpLowering {}

impl RewritePattern for ConstantOpLowering {
    fn name(&self) -> String {
        "ConstantOpLowering".to_string()
    }

    fn match_op(&self, ctx: &Context, op: Ptr<Operation>) -> Result<bool, anyhow::Error> {
        Ok(op
            .deref(ctx)
            .get_op(ctx)
            .downcast_ref::<wasm::ops::ConstantOp>()
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
        if let Some(const_op) = opop.downcast_ref::<wasm::ops::ConstantOp>() {
            let value = const_op.get_value(ctx);
            if let Ok(value_attr) = value.downcast::<IntegerAttr>() {
                let value = oxfoi_field_elem_from_int(ctx, *value_attr)?;
                let const_op = miden::ops::ConstantOp::new_unlinked(ctx, value);
                rewriter.replace_op_with(ctx, op, const_op.get_operation())?;
            } else {
                return Err(anyhow!("only integer constants are supported"));
            }
        }
        Ok(())
    }
}
