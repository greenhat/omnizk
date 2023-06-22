use anyhow::anyhow;
use apint::ApInt;
use apint::Int;
use apint::Width;
use miden::attributes::FieldElem;
use miden::attributes::FieldElemAttr;
use miden::types::FieldElemType;
use ozk_miden_dialect as miden;
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
use winter_math::StarkField;

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
                let field_elem_type = FieldElemType::get(ctx);
                if value_attr.get_type() == i32_type(ctx) {
                    let value: ApInt = (*value_attr).into();
                    let attr = FieldElemAttr::create(field_elem_type, apint64_to_field_elem(value));
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

fn apint64_to_field_elem(value: ApInt) -> FieldElem {
    assert!(value.width() <= 64.into());
    let i = Int::from(value);
    #[allow(clippy::expect_used)]
    let raw = i.try_to_i64().expect("64-bit integer");
    felt_i64(raw)
}

fn felt_i64(v: i64) -> FieldElem {
    if v < 0 {
        FieldElem::new(FieldElem::MODULUS - v.unsigned_abs())
    } else {
        FieldElem::new(v as u64)
    }
}
