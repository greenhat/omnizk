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
    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::new(WasmGlobalSetToMem::new(self.start_addr)));
        patterns.add(Box::new(WasmGlobalGetToMem::new(self.start_addr)));
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

const MAX_GLOBAL_VAR_SIZE_BYTES: u32 = 8; // i64

pub struct WasmGlobalSetToMem {
    start_addr: MemAddress,
}

impl WasmGlobalSetToMem {
    pub fn new(start_addr: MemAddress) -> Self {
        Self { start_addr }
    }
}

impl RewritePattern for WasmGlobalSetToMem {
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
        let offset: u32 = u32::from(global_set_op.get_index(ctx)) * MAX_GLOBAL_VAR_SIZE_BYTES;
        let address = u32::from(self.start_addr) - offset;
        let constant_op = wasm::ConstantOp::new_i32_unlinked(ctx, address as i32);
        let i64store_op = wasm::StoreOp::new_unlinked(ctx, wasm::MemAccessOpValueType::I64);
        rewriter.insert_before(ctx, constant_op.get_operation())?;
        let swap_op = ozk::SwapOp::new_unlinked(ctx, Ord16::ST1);
        rewriter.insert_before(ctx, swap_op.get_operation())?;
        rewriter.replace_op_with(
            ctx,
            global_set_op.get_operation(),
            i64store_op.get_operation(),
        )?;
        Ok(())
    }
}

pub struct WasmGlobalGetToMem {
    start_addr: MemAddress,
}

impl WasmGlobalGetToMem {
    pub fn new(start_addr: MemAddress) -> Self {
        Self { start_addr }
    }
}

impl RewritePattern for WasmGlobalGetToMem {
    fn match_op(&self, ctx: &Context, op: Ptr<Operation>) -> Result<bool, anyhow::Error> {
        Ok(op
            .deref(ctx)
            .get_op(ctx)
            .downcast_ref::<wasm::GlobalGetOp>()
            .is_some())
    }

    #[allow(clippy::panic)]
    fn rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        rewriter: &mut dyn PatternRewriter,
    ) -> Result<(), anyhow::Error> {
        let Ok(global_get_op) = op
            .deref(ctx)
            .get_op(ctx)
            .downcast::<wasm::GlobalGetOp>() else {
            panic!("unexpected op {}", op.deref(ctx).with_ctx(ctx));
        };
        let offset: u32 = u32::from(global_get_op.get_index(ctx)) * MAX_GLOBAL_VAR_SIZE_BYTES;
        let address = u32::from(self.start_addr) - offset;
        let constant_op = wasm::ConstantOp::new_i32_unlinked(ctx, address as i32);
        let i64load_op = wasm::LoadOp::new_unlinked(ctx, wasm::MemAccessOpValueType::I64);
        rewriter.insert_before(ctx, constant_op.get_operation())?;
        rewriter.replace_op_with(
            ctx,
            global_get_op.get_operation(),
            i64load_op.get_operation(),
        )?;
        Ok(())
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {

    use expect_test::expect;

    use crate::tests_util::check_wasm_pass;

    use super::*;

    #[test]
    fn globals_get_set() {
        let pass = WasmGlobalsToMemPass {
            start_addr: 0x1000.into(),
        };
        check_wasm_pass(
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
                        wasm.const 0x1000: si32
                        wasm.load I64
                        wasm.return
                    }
                }"#]],
        );
    }
}
