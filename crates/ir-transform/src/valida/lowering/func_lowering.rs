use ozk_valida_dialect as valida;
use ozk_wasm_dialect as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::dialects::builtin::op_interfaces::SymbolOpInterface;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::operation::WalkOrder;
use pliron::operation::WalkResult;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;
use valida::types::Operands;
use wasm::op_interfaces::TrackedStackDepth;
use wasm::ops::LocalGetOp;
use wasm::ops::ReturnOp;

use crate::valida::fp_from_wasm_stack;

#[derive(Default)]
pub struct WasmToValidaFuncLoweringPass;

impl Pass for WasmToValidaFuncLoweringPass {
    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<FuncOpLowering>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct FuncOpLowering {}

impl RewritePattern for FuncOpLowering {
    fn match_and_rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        rewriter: &mut dyn PatternRewriter,
    ) -> Result<bool, anyhow::Error> {
        let opop = &op.deref(ctx).get_op(ctx);
        let Some(wasm_func_op) = opop.downcast_ref::<wasm::ops::FuncOp>() else {
            return Ok(false);
        };
        // TODO: extract local.gets conversion into a function
        let mut local_get_ops = Vec::new();
        wasm_func_op.get_operation().walk_only::<LocalGetOp>(
            ctx,
            WalkOrder::PostOrder,
            &mut |op| {
                local_get_ops.push(*op);
                WalkResult::Advance
            },
        );

        let fp_func_first_arg: i32 = 12;
        for local_get_op in local_get_ops {
            let index: u32 = local_get_op.get_index(ctx).into();
            if index < wasm_func_op.get_type_typed(ctx).get_inputs().len() as u32 {
                // this is function paramter
                let wasm_stack_depth_before_op = local_get_op.get_stack_depth(ctx);
                let from_fp: i32 = fp_func_first_arg + index as i32 * 4;
                let to_fp: i32 = fp_from_wasm_stack(wasm_stack_depth_before_op.next()).into();
                let sw_op = valida::ops::SwOp::new_unlinked(
                    ctx,
                    Operands::from_i32(0, to_fp, from_fp, 0, 0),
                );
                rewriter.replace_op_with(
                    ctx,
                    local_get_op.get_operation(),
                    sw_op.get_operation(),
                )?;
            }
        }

        // TODO: extract return conversion into a function
        let mut return_ops = Vec::new();
        wasm_func_op
            .get_operation()
            .walk_only::<ReturnOp>(ctx, WalkOrder::PostOrder, &mut |op| {
                return_ops.push(*op);
                WalkResult::Advance
            });
        for return_op in return_ops {
            // TODO: check func signature if there is a return value (after I/O is implemented)
            // if wasm_func_op.get_type_typed(ctx).get_results().len() == 1 {
            let wasm_stack_depth_before_op = return_op.get_stack_depth(ctx);
            let last_stack_value_fp_offset = fp_from_wasm_stack(wasm_stack_depth_before_op);
            let return_value_fp_offset = 4;
            let sw_op = valida::ops::SwOp::new_unlinked(
                ctx,
                Operands::from_i32(
                    0,
                    return_value_fp_offset,
                    last_stack_value_fp_offset.into(),
                    0,
                    0,
                ),
            );
            rewriter.set_insertion_point(return_op.get_operation());
            rewriter.insert_before(ctx, sw_op.get_operation())?;
            // } else {
            //     todo!("wasm.func -> valida: multiple return values are not supported yet");
            // }
            let ret_op = valida::ops::JalvOp::new_return_pseudo_op(ctx);
            rewriter.replace_op_with(ctx, return_op.get_operation(), ret_op.get_operation())?;
        }

        let func_op = valida::ops::FuncOp::new_unlinked(ctx, wasm_func_op.get_symbol_name(ctx));
        for op in wasm_func_op.op_iter(ctx) {
            op.unlink(ctx);
            op.insert_at_back(func_op.get_entry_block(ctx), ctx);
        }
        rewriter.replace_op_with(ctx, wasm_func_op.get_operation(), func_op.get_operation())?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::tests_util::check_wasm_valida_pass;
    use crate::wasm::track_stack_depth::WasmTrackStackDepthPass;

    use super::*;

    #[test]
    fn func_op_lowering() {
        check_wasm_valida_pass(
            vec![
                Box::<WasmTrackStackDepthPass>::default(),
                Box::<WasmToValidaFuncLoweringPass>::default(),
            ],
            r#"
(module
    (start $main)
    (func $add (param i32 i32) (result i32)
        get_local 0
        get_local 1
        i32.add
        return)
    (func $main
        i32.const 3
        i32.const 4
        call $add
        return)
)
        "#,
            expect![[r#"
                wasm.module @module_name {
                  block_2_0():
                    valida.func @add {
                      entry():
                        valida.sw 0 -4(fp) 12(fp) 0 0
                        valida.sw 0 -8(fp) 16(fp) 0 0
                        wasm.add
                        valida.sw 0 4(fp) -4(fp) 0 0
                        valida.jalv -4(fp) 0(fp) 8(fp) 0 0
                    }
                    valida.func @main {
                      entry():
                        wasm.const 0x3: si32
                        wasm.const 0x4: si32
                        wasm.call 0
                        valida.sw 0 4(fp) -8(fp) 0 0
                        valida.jalv -4(fp) 0(fp) 8(fp) 0 0
                    }
                }"#]],
        )
    }
}
