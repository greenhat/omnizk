#![allow(dead_code)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use ozk_frontend_wasm::WasmFrontendConfig;
use ozk_wasm_dialect as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin;
use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use pliron::operation::Operation;
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

pub fn check_wasm_valida_passes(
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
    let wrapper_module = wrap_in_builtin_module(&mut ctx, wasm_module_op);
    for pass in passes {
        eprintln!(" before {} pass:", pass.name());
        eprintln!("{}", wrapper_module.with_ctx(&ctx));
        pass.run_on_operation(&mut ctx, wrapper_module.get_operation())
            .unwrap();
    }
    let unwrapped_op = unwrap_from_builtin_module(&mut ctx, wrapper_module);
    expected.assert_eq(unwrapped_op.with_ctx(&ctx).to_string().as_str());
}

fn wrap_in_builtin_module(
    ctx: &mut Context,
    wasm_module: wasm::ops::ModuleOp,
) -> builtin::ops::ModuleOp {
    let wrapper_module = builtin::ops::ModuleOp::new(ctx, "wrapper");
    wasm_module
        .get_operation()
        .insert_at_back(wrapper_module.get_body(ctx, 0), ctx);
    wrapper_module
}

fn unwrap_from_builtin_module(
    ctx: &mut Context,
    builtin_module: builtin::ops::ModuleOp,
) -> Ptr<Operation> {
    let inner_module = builtin_module
        .get_body(ctx, 0)
        .deref(ctx)
        .iter(ctx)
        .collect::<Vec<Ptr<Operation>>>()
        .first()
        .cloned()
        .unwrap();
    inner_module
}
