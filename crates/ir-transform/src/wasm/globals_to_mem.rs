use ozk_wasm_dialect::ops as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::operation::Operation;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;
use pliron::with_context::AttachContext;

#[derive(Default)]
pub struct WasmGlobalsToMemPass;

impl Pass for WasmGlobalsToMemPass {
    fn name(&self) -> &str {
        "WasmGlobalsToMemPass"
    }

    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<WasmGlobalsToMem>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct WasmGlobalsToMem;

impl RewritePattern for WasmGlobalsToMem {
    fn name(&self) -> String {
        "WasmGlobalsToMem".to_string()
    }

    fn match_op(&self, ctx: &Context, op: Ptr<Operation>) -> Result<bool, anyhow::Error> {
        Ok(op
            .deref(ctx)
            .get_op(ctx)
            .downcast_ref::<wasm::ModuleOp>()
            .is_some())
    }

    #[allow(clippy::panic)]
    fn rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        _rewriter: &mut dyn PatternRewriter,
    ) -> Result<(), anyhow::Error> {
        let Ok(_module_op) = op
            .deref(ctx)
            .get_op(ctx)
            .downcast::<wasm::ModuleOp>() else {
            panic!("unexpected op {}", op.deref(ctx).with_ctx(ctx));
        };
        Ok(())
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {

    use expect_test::expect;
    use ozk_frontend_wasm::WasmFrontendConfig;
    use pliron::op::Op;

    use super::*;

    pub fn check_pass<T: Pass>(pass: &T, wat: &str, expected: expect_test::Expect) {
        let source = wat::parse_str(wat).unwrap();
        let mut ctx = Context::default();
        let frontend_config = WasmFrontendConfig::default();
        ozk_wasm_dialect::register(&mut ctx);
        frontend_config.register(&mut ctx);
        let wasm_module_op =
            ozk_frontend_wasm::parse_module(&mut ctx, &source, &frontend_config).unwrap();
        pass.run_on_operation(&mut ctx, wasm_module_op.get_operation())
            .unwrap();
        expected.assert_eq(wasm_module_op.with_ctx(&ctx).to_string().as_str());
    }

    #[test]
    fn globals_get_set() {
        check_pass(
            &WasmGlobalsToMemPass::default(),
            r#"
(module
    (type (;2;) (func))
    (global $MyGlobalVal (mut i32) i32.const 42)
    (export "main" (func $main))
    (start $main)
    (func $main
        i32.const 9
        global.set $MyGlobalVal
        global.get $MyGlobalVal
        return)
)
"#,
            expect![[r#"
                wasm.module @module_name {
                  block_1_0():
                    wasm.func @main() -> () {
                      entry():
                        wasm.const 0x9: si32
                        MEM_ACCESS_CALLS
                        wasm.return
                    }
                }"#]],
        );
    }
}
