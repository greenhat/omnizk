#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::anyhow;
use ozk_valida_dialect as valida;
use ozk_wasm_dialect as wasm;
use ozk_wasm_dialect::op_interfaces::TrackedStackDepth;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;
use valida::types::Mersenne31;
use valida::types::Operands;

use crate::valida::fp_from_wasm_stack;

#[derive(Default)]
pub struct WasmToValidaArithLoweringPass;

impl Pass for WasmToValidaArithLoweringPass {
    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<ConstantOpLowering>::default());
        patterns.add(Box::<ArithOpLowering>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

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
                // TODO: Note that because a full 32-bit value does not fit within one field element, we assume that values have been decomposed into 4 8-byte elements
                let value: Mersenne31 = value_attr.as_ref().try_into()?;
                let wasm_stack_depth_before_op = const_op.get_stack_depth(ctx);
                let a_fp = fp_from_wasm_stack(wasm_stack_depth_before_op.next());
                let a = a_fp.into();
                let b = 0;
                let c = 0;
                let d = 0;
                let imm_op = valida::ops::Imm32Op::new_unlinked(
                    ctx,
                    Operands::from_i32(a, b, c, d, value.as_i32()),
                );
                rewriter.replace_op_with(ctx, op, imm_op.get_operation())?;
            } else {
                return Err(anyhow!("only integer constants are supported"));
            }
        }
        Ok(())
    }
}

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
        if let Some(wasm_add_op) = opop.downcast_ref::<wasm::ops::AddOp>() {
            let wasm_stack_depth_before_op = wasm_add_op.get_stack_depth(ctx);
            // add wasm pops 2 values and pushes 1,
            // so the result ends up on the first argument stack slot
            let result_fp = fp_from_wasm_stack(wasm_stack_depth_before_op.minus1());
            let arg1_fp = fp_from_wasm_stack(wasm_stack_depth_before_op.top());
            let arg2_fp = fp_from_wasm_stack(wasm_stack_depth_before_op.minus1());
            let add_op =
                valida::ops::AddOp::new(ctx, result_fp.into(), arg1_fp.into(), arg2_fp.into());
            rewriter.replace_op_with(ctx, op, add_op.get_operation())?;
        }
        Ok(())
    }
}
