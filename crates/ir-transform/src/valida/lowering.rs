use ozk_valida_dialect::VALIDA_DIALECT;
use ozk_wasm_dialect::WASM_DIALECT;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect::Dialect;
use pliron::dialect::DialectName;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::operation::Operation;
use pliron::pass::Pass;
use pliron::rewrite::RewritePatternSet;

// use self::arith_op_lowering::ArithOpLowering;
use self::constant_op_lowering::ConstantOpLowering;

// pub mod arith_op_lowering;
pub mod constant_op_lowering;

#[derive(Default)]
pub struct WasmToValidaArithLoweringPass;

impl Pass for WasmToValidaArithLoweringPass {
    fn name(&self) -> &str {
        "WasmToValidaArithLoweringPass"
    }

    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let mut target = ConversionTarget::default();
        target.add_legal_dialect(VALIDA_DIALECT(ctx));
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<ConstantOpLowering>::default());
        // patterns.add(Box::<ArithOpLowering>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

/// The pass that ensures there are no Wasm ops left.
#[derive(Default)]
pub struct WasmToValidaFinalLoweringPass;

impl Pass for WasmToValidaFinalLoweringPass {
    fn name(&self) -> &str {
        "WasmToValidaFinalLoweringPass"
    }

    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let mut target = ConversionTarget::default();
        target.add_illegal_dialect(WASM_DIALECT(ctx));
        target.add_legal_dialect(VALIDA_DIALECT(ctx));
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
