use anyhow::anyhow;
use ozk_ozk_dialect::attributes::p231m1_field_elem_from_int;
use ozk_ozk_dialect::attributes::p231m1_field_elem_from_int_attr;
use ozk_valida_dialect as valida;
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
                let value = p231m1_field_elem_from_int_attr(ctx, *value_attr)?;
                todo!("cell ofsset can be determined only by analyzing the entire function(track stack depth?");
                // TODO: moreover, we need to know stack depth in runtime for every wasm op
                // prepend every wasm op with an op that will update the current stack depth stored in the local var?
                // and define for each wasm op how many elements it pops from and pushes to the stack
                let cell_offset = p231m1_field_elem_from_int(ctx, 0);
                let zero = p231m1_field_elem_from_int(ctx, 0);
                let b = zero.clone();
                let c = zero.clone();
                let d = zero;
                let imm_op = valida::ops::Imm32Op::new_unlinked(ctx, cell_offset, b, c, d, value);
                rewriter.replace_op_with(ctx, op, imm_op.get_operation())?;
            } else {
                return Err(anyhow!("only integer constants are supported"));
            }
        }
        Ok(())
    }
}