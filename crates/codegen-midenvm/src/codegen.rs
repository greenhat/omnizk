mod inst_buf;
pub use inst_buf::InstBuffer;
mod emit;
pub use emit::*;
mod miden_inst;
pub use miden_inst::*;

#[allow(clippy::unwrap_used)]
#[allow(unused_variables)]
#[cfg(test)]
mod tests {

    use expect_test::expect;
    use pliron::context::Context;
    use pliron::dialects::builtin;

    pub(crate) fn setup_context_dialects() -> Context {
        let mut ctx = Context::new();
        ozk_wasm_dialect::register(&mut ctx);
        builtin::register(&mut ctx);
        ozk_miden_dialect::register(&mut ctx);
        ctx
    }

    #[cfg(test)]
    fn check(input: &str, expected_tree: expect_test::Expect) {
        use c2zk_frontend::translate;
        use c2zk_frontend::FrontendConfig;
        use ozk_frontend_wasm::WasmFrontendConfig;
        use pliron::context::Ptr;
        use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
        use pliron::linked_list::ContainsLinkedList;
        use pliron::op::Op;
        use pliron::operation::Operation;
        use pliron::with_context::AttachContext;

        use crate::MidenTargetConfig;

        let source = wat::parse_str(input).unwrap();
        let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
        let mut ctx = setup_context_dialects();
        let wasm_module_op = translate(&mut ctx, &source, frontend).unwrap();
        let wrapper_module = builtin::ops::ModuleOp::new(&mut ctx, "wrapper");
        wasm_module_op
            .get_operation()
            .insert_at_back(wrapper_module.get_body(&ctx, 0), &ctx);
        let miden_target_config = MidenTargetConfig::default();
        miden_target_config
            .pass_manager
            .run(&mut ctx, wrapper_module.get_operation())
            .unwrap();
        let miden_prog = wrapper_module
            .get_body(&ctx, 0)
            .deref(&ctx)
            .iter(&ctx)
            .collect::<Vec<Ptr<Operation>>>()
            .first()
            .cloned()
            .unwrap();
        expected_tree.assert_eq(miden_prog.with_ctx(&ctx).to_string().as_str());
    }

    #[test]
    fn test_smoke() {
        check(
            r#"
(module 
    (start $f1)
    (func $f1 
        i32.const 1
        return)
)"#,
            expect![[r#"
                miden.program {
                  block_3_0():
                    miden.proc @f1 {
                      entry():
                        miden.constant 1: felt
                    }
                }"#]],
        );
    }
}
