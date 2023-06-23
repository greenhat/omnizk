use anyhow::anyhow;
use ozk_miden_dialect as miden;
use ozk_ozk_dialect::attributes::FieldElemAttr;
use ozk_ozk_dialect::attributes::apint64_to_field_elem;
use ozk_ozk_dialect::types::Field;
use ozk_ozk_dialect::types::FieldElemType;
use ozk_wasm_dialect as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::attr_interfaces::TypedAttrInterface;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use wasm::types::i32_type;

#[derive(Default)]
pub struct ConstantOpLowering {}

impl RewritePattern for ConstantOpLowering {
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
                let field_elem_type = FieldElemType::get(ctx, Field::Oxfoi);
                if value_attr.get_type() == i32_type(ctx) {
                    let attr = FieldElemAttr::create(
                        field_elem_type,
                        apint64_to_field_elem((*value_attr).into()),
                    );
                    let const_op = miden::ops::ConstantOp::new_unlinked(ctx, attr);
                    rewriter.replace_op_with(ctx, op, const_op.get_operation())?;
                } else {
                    return Err(anyhow!("only 32-bit integers are supported"));
                }
            } else {
                return Err(anyhow!("only integer constants are supported"));
            }
        }
        Ok(())
    }
}
