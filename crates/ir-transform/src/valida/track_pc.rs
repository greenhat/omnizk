use ozk_valida_dialect::op_interfaces::CustomProgramCountChange;
use ozk_valida_dialect::op_interfaces::TrackedProgramCounter;
use ozk_valida_dialect::ops::ProgramOp;
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

#[derive(Default)]
pub struct ValidaTrackProgramCounterPass;

impl Pass for ValidaTrackProgramCounterPass {
    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // TODO: leads to pattern NOT called for ProgramOp
        // target.add_illegal_dialect(WASM_DIALECT(ctx));
        // target.add_illegal_dialect(OZK_DIALECT(ctx));
        // target.add_legal_dialect(VALIDA_DIALECT(ctx));
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<ValidaTrackProgramCounter>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct ValidaTrackProgramCounter;

impl RewritePattern for ValidaTrackProgramCounter {
    fn match_and_rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        _rewriter: &mut dyn PatternRewriter,
    ) -> Result<bool, anyhow::Error> {
        let Ok(program_op) = op
            .deref(ctx)
            .get_op(ctx)
            .downcast::<ProgramOp>() else {
            return Ok(false);
        };
        let mut ops = Vec::new();
        program_op
            .get_operation()
            .walk(ctx, WalkOrder::PreOrder, &mut |op| {
                ops.push(op);
                WalkResult::Advance
            });
        let mut pc: u32 = 0;
        for op in ops {
            let op_op = op.deref(ctx).get_op(ctx);
            if let Some(tracked_op) = op_cast::<dyn TrackedProgramCounter>(op_op.as_ref()) {
                tracked_op.set_pc(ctx, pc.into());
            }
            if let Some(custom_pc_change_op) =
                op_cast::<dyn CustomProgramCountChange>(op_op.as_ref())
            {
                pc = (pc as i32 + custom_pc_change_op.get_pc_change(ctx)) as u32;
            } else {
                pc += 1;
            }
        }
        Ok(true)
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {

    use expect_test::expect;

    use crate::tests_util::check_wasm_valida_passes;
    use crate::valida::lowering::arith_op_lowering::WasmToValidaArithLoweringPass;
    use crate::valida::lowering::func_lowering::WasmToValidaFuncLoweringPass;
    use crate::valida::lowering::module_lowering::WasmToValidaModuleLoweringPass;
    use crate::wasm::resolve_call_op::WasmCallOpToOzkCallOpPass;
    use crate::wasm::track_stack_depth::WasmTrackStackDepthPass;

    use super::*;

    #[test]
    fn smoke_track_pc() {
        check_wasm_valida_passes(
            vec![
                Box::<WasmCallOpToOzkCallOpPass>::default(),
                Box::new(WasmTrackStackDepthPass::new_reserve_space_for_locals()),
                Box::<WasmToValidaArithLoweringPass>::default(),
                Box::<WasmToValidaFuncLoweringPass>::default(),
                Box::<WasmToValidaModuleLoweringPass>::default(),
                Box::<ValidaTrackProgramCounterPass>::default(),
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
                valida.program {
                  entry():
                    valida.imm32 -24(fp) 0 0 0 28
                    valida.jalsym -28(fp) main -28 0 0
                    valida.sw 0 4(fp) -20(fp) 0 0
                    valida.exit
                  block_5_2():
                    valida.func @add pc=4 {
                      entry():
                        valida.sw 0 -4(fp) 12(fp) 0 0
                        valida.sw 0 -8(fp) 16(fp) 0 0
                        valida.add -4(fp) -8(fp) -4(fp) 0 0
                        valida.sw 0 16(fp) -4(fp) 0 0
                        valida.jalv -4(fp) 0(fp) 4(fp) 0 0
                    }
                    valida.func @main pc=9 {
                      entry():
                        valida.imm32 -4(fp) 0 0 0 3
                        valida.imm32 -8(fp) 0 0 0 4
                        valida.imm32 -16(fp) 0 0 0 20
                        valida.jalsym -20(fp) add -20 0 0
                        valida.sw 0 8(fp) -4(fp) 0 0
                        valida.jalv -4(fp) 0(fp) 4(fp) 0 0
                    }
                }"#]],
        )
    }
}
