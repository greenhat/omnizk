#![allow(dead_code)]
#![allow(clippy::unwrap_used)]

use ozk_frontend_wasm::WasmFrontendConfig;
use pliron::context::Context;
use pliron::op::Op;
use pliron::pass::Pass;
use pliron::with_context::AttachContext;

pub fn check_wasm_pass<T: Pass>(pass: &T, wat: &str, expected: expect_test::Expect) {
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

pub fn check_wasm_valida_pass(
    passes: Vec<Box<dyn Pass>>,
    wat: &str,
    expected: expect_test::Expect,
) {
    let source = wat::parse_str(wat).unwrap();
    let mut ctx = Context::default();
    let frontend_config = WasmFrontendConfig::default();
    ozk_wasm_dialect::register(&mut ctx);
    ozk_ozk_dialect::register(&mut ctx);
    ozk_valida_dialect::register(&mut ctx);
    frontend_config.register(&mut ctx);
    let wasm_module_op =
        ozk_frontend_wasm::parse_module(&mut ctx, &source, &frontend_config).unwrap();
    for pass in passes {
        pass.run_on_operation(&mut ctx, wasm_module_op.get_operation())
            .unwrap();
    }
    expected.assert_eq(wasm_module_op.with_ctx(&ctx).to_string().as_str());
}
