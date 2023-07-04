use ozk_ozk_dialect::ops as ozk;
use ozk_ozk_dialect::ord_n::Ord16;
use ozk_wasm_dialect::ops as wasm;
use ozk_wasm_dialect::types::MemAddress;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;
use pliron::with_context::AttachContext;

pub struct WasmGlobalsToMemPass {
    start_addr: MemAddress,
}

impl WasmGlobalsToMemPass {
    pub fn new(start_addr: MemAddress) -> Self {
        Self { start_addr }
    }
}

impl Pass for WasmGlobalsToMemPass {
    fn name(&self) -> &str {
        "WasmGlobalsToMemPass"
    }

    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::new(WasmGlobalSetToMem::new(self.start_addr)));
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

pub struct WasmGlobalSetToMem {
    start_addr: MemAddress,
}

impl WasmGlobalSetToMem {
    pub fn new(start_addr: MemAddress) -> Self {
        Self { start_addr }
    }
}

impl RewritePattern for WasmGlobalSetToMem {
    fn name(&self) -> String {
        "WasmGlobalSetToMem".to_string()
    }

    fn match_op(&self, ctx: &Context, op: Ptr<Operation>) -> Result<bool, anyhow::Error> {
        Ok(op
            .deref(ctx)
            .get_op(ctx)
            .downcast_ref::<wasm::GlobalSetOp>()
            .is_some())
    }

    #[allow(clippy::panic)]
    fn rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        rewriter: &mut dyn PatternRewriter,
    ) -> Result<(), anyhow::Error> {
        let Ok(global_set_op) = op
            .deref(ctx)
            .get_op(ctx)
            .downcast::<wasm::GlobalSetOp>() else {
            panic!("unexpected op {}", op.deref(ctx).with_ctx(ctx));
        };
        let max_global_var_size_bytes = 8; // i64
        let offset: u32 = u32::from(global_set_op.get_index(ctx)) * max_global_var_size_bytes;
        let address = u32::from(self.start_addr) - offset;
        let constant_op = wasm::ConstantOp::new_i32_unlinked(ctx, address as i32);
        let i64store_op = wasm::StoreOp::new_unlinked(ctx, wasm::StoreOpValueType::I64);
        rewriter.replace_op_with(
            ctx,
            global_set_op.get_operation(),
            i64store_op.get_operation(),
        )?;
        // TODO: add rewriter.insert_after/before?
        rewriter.set_insertion_point(i64store_op.get_operation());
        rewriter.insert(ctx, constant_op.get_operation())?;
        let swap_op = ozk::SwapOp::new_unlinked(ctx, Ord16::ST1);
        rewriter.insert(ctx, swap_op.get_operation())?;
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

    // TODO: move to crate's test utils
    pub fn check_pass<T: Pass>(pass: &T, wat: &str, expected: expect_test::Expect) {
        let source = wat::parse_str(wat).unwrap();
        let mut ctx = Context::default();
        let frontend_config = WasmFrontendConfig::default();
        ozk_wasm_dialect::register(&mut ctx);
        ozk_ozk_dialect::register(&mut ctx);
        frontend_config.register(&mut ctx);
        let wasm_module_op =
            ozk_frontend_wasm::parse_module(&mut ctx, &source, &frontend_config).unwrap();
        pass.run_on_operation(&mut ctx, wasm_module_op.get_operation())
            .unwrap();
        expected.assert_eq(wasm_module_op.with_ctx(&ctx).to_string().as_str());
    }

    #[test]
    fn globals_get_set() {
        let pass = WasmGlobalsToMemPass {
            start_addr: 0x1000.into(),
        };
        check_pass(
            &pass,
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
                        wasm.const 0x1000: si32
                        ozk.swap 1
                        wasm.store I64
                        wasm.global.get 0x0: ui32
                        wasm.return
                    }
                }"#]],
        );
    }
}
