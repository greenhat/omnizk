use ozk_miden_dialect::MIDEN_DIALECT;
use ozk_wasm_dialect::WASM_DIALECT;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect::Dialect;
use pliron::dialect::DialectName;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::operation::Operation;
use pliron::pass::Pass;
use pliron::pass::PassError;
use pliron::rewrite::RewritePatternSet;

use self::constant_op_lowering::ConstantOpLowering;

mod cf_lowering;
pub use cf_lowering::WasmToMidenCFLoweringPass;

pub mod constant_op_lowering;

#[derive(Default)]
pub struct WasmToMidenArithLoweringPass;

impl Pass for WasmToMidenArithLoweringPass {
    fn name(&self) -> &str {
        "WasmToMidenArithLoweringPass"
    }

    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), PassError> {
        let mut target = ConversionTarget::default();
        target.add_legal_dialect(MIDEN_DIALECT(ctx));
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<ConstantOpLowering>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

/// The pass that ensures there are no Wasm ops left.
#[derive(Default)]
pub struct WasmToMidenFinalLoweringPass;

impl Pass for WasmToMidenFinalLoweringPass {
    fn name(&self) -> &str {
        "WasmToMidenFinalLoweringPass"
    }

    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), PassError> {
        let mut target = ConversionTarget::default();
        target.add_illegal_dialect(WASM_DIALECT(ctx));
        target.add_legal_dialect(MIDEN_DIALECT(ctx));
        #[allow(clippy::expect_used)]
        target.add_legal_dialect(
            Dialect::get_ref(ctx, DialectName::new("builtin"))
                .expect("builtin dialect not registered"),
        );
        let patterns = RewritePatternSet::default();
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}
