use ozk_frontend_wasm::func_builder::FuncBuilder;
use ozk_ozk_dialect::types::u32_type;
use ozk_wasm_dialect::ops as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::dialects::builtin::types::FunctionType;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::operation::WalkOrder;
use pliron::operation::WalkResult;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;
use pliron::with_context::AttachContext;

const GLOBALS_GET_FUNC_NAME: &str = "omnizk_globals_get";
const GLOBALS_SET_FUNC_NAME: &str = "omnizk_globals_set";

pub struct WasmGlobalsToMemPass {
    start_addr: i32,
}

impl WasmGlobalsToMemPass {
    pub fn new(start_addr: i32) -> Self {
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
        patterns.add(Box::new(WasmGlobalsToMem::new(self.start_addr)));
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

pub struct WasmGlobalsToMem {
    start_addr: i32,
}

impl WasmGlobalsToMem {
    pub fn new(start_addr: i32) -> Self {
        Self { start_addr }
    }
}

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
        rewriter: &mut dyn PatternRewriter,
    ) -> Result<(), anyhow::Error> {
        let Ok(module_op) = op
            .deref(ctx)
            .get_op(ctx)
            .downcast::<wasm::ModuleOp>() else {
            panic!("unexpected op {}", op.deref(ctx).with_ctx(ctx));
        };

        let mut global_ops: Vec<Ptr<Operation>> = vec![];
        op.walk(ctx, WalkOrder::PreOrder, &mut |op| {
            if op
                .deref(ctx)
                .get_op(ctx)
                .downcast_ref::<wasm::GlobalGetOp>()
                .is_some()
            {
                global_ops.push(op);
            };
            if op
                .deref(ctx)
                .get_op(ctx)
                .downcast_ref::<wasm::GlobalSetOp>()
                .is_some()
            {
                global_ops.push(op);
            };
            WalkResult::Advance
        });

        if global_ops.is_empty() {
            return Ok(());
        }

        let global_set_func_op = global_set_func(ctx, self.start_addr);
        let global_set_func_index = module_op
            .get_func_index(ctx, GLOBALS_SET_FUNC_NAME.into())
            .unwrap_or_else(|| module_op.append_function(ctx, global_set_func_op));
        for op in global_ops {
            let deref_op = &op.deref(ctx).get_op(ctx);
            let Some(global_set_op) =  deref_op
                .downcast_ref::<wasm::GlobalSetOp>()
               else
            {
                panic!();
            };
            let global_set_op_index = global_set_op.get_index(ctx);
            let constant_op = wasm::ConstantOp::new_unlinked(ctx, global_set_op_index);
            let call_op = wasm::CallOp::new_unlinked(ctx, global_set_func_index);
            rewriter.replace_op_with(
                ctx,
                global_set_op.get_operation(),
                call_op.get_operation(),
            )?;
            rewriter.set_insertion_point(call_op.get_operation());
            rewriter.insert(ctx, constant_op.get_operation())?;
        }
        Ok(())
    }
}

fn global_get_func(ctx: &mut Context, start_addr: i32) -> wasm::FuncOp {
    todo!()
}

#[allow(clippy::unwrap_used)]
fn global_set_func(ctx: &mut Context, start_addr: i32) -> wasm::FuncOp {
    let mut func_builder = FuncBuilder::new(ctx, GLOBALS_SET_FUNC_NAME.into());
    let inputs = vec![u32_type(ctx)];
    let sig = FunctionType::get(ctx, inputs, vec![]);
    func_builder.set_signature(sig);
    func_builder.build(ctx).unwrap()
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
        frontend_config.register(&mut ctx);
        let wasm_module_op =
            ozk_frontend_wasm::parse_module(&mut ctx, &source, &frontend_config).unwrap();
        pass.run_on_operation(&mut ctx, wasm_module_op.get_operation())
            .unwrap();
        expected.assert_eq(wasm_module_op.with_ctx(&ctx).to_string().as_str());
    }

    #[test]
    fn globals_get_set() {
        let pass = WasmGlobalsToMemPass { start_addr: 0x1000 };
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
                        wasm.global.set 0x0: ui32
                        wasm.global.get 0x0: ui32
                        wasm.return
                    }
                }"#]],
        );
    }
}
