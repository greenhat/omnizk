use ozk_miden_dialect::MIDEN_DIALECT;
use ozk_wasm_dialect::WASM_DIALECT;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::error::CompilerError;
use pliron::operation::Operation;
use pliron::pass::Pass;
use pliron::rewrite::RewritePatternSet;

use self::cf_lowering::ControlFlowLowering;
use self::constant_op_lowering::ConstantOpLowering;

mod cf_lowering;
mod constant_op_lowering;

#[derive(Default)]
pub struct WasmToMidenLoweringPass {}

impl Pass for WasmToMidenLoweringPass {
    fn name(&self) -> &str {
        "WasmToMidenLoweringPass"
    }

    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), CompilerError> {
        let mut target = ConversionTarget::default();
        target.add_illegal_dialect(WASM_DIALECT(ctx));
        target.add_legal_dialect(MIDEN_DIALECT(ctx));
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<ControlFlowLowering>::default());
        patterns.add(Box::<ConstantOpLowering>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}
