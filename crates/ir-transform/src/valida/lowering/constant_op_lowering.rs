#![allow(unused_variables)]
#![allow(dead_code)]

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
use wasm::op_interfaces::TrackedStackDepth;

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
                // TODO: add pass to set the current stack depth for all ops in a function
                // walk the functions ops and attach to every wasm op the stack depth before that op
                // i.e. for add op if the stack deptch is N we set valida operands to fp(N) and fp(N - 1)
                // and set the result to fp((N - 2) + 1).
                let wasm_stack_depth_before_op = const_op.get_stack_depth(ctx);
                let cell_offset_raw = -((wasm_stack_depth_before_op as i32 + 1) * 4);
                let cell_offset = p231m1_field_elem_from_int(ctx, cell_offset_raw);
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
