use std::collections::HashMap;

use c2zk_codegen_shared::func_index_to_label;
use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::Module;

mod inst_buf;
pub use inst_buf::InstBuffer;
mod emit;
pub use emit::*;
mod miden_inst;
pub use miden_inst::*;

use crate::MidenError;
use crate::MidenTargetConfig;

#[allow(dead_code)]
#[cfg(test)]
mod sem_tests;

pub fn compile_module(
    module: Module,
    config: &MidenTargetConfig,
) -> Result<InstBuffer, MidenError> {
    let mut sink = InstBuffer::new(config);
    let func_names = module.func_names();
    let builder = MidenAssemblyBuilder::new();
    let start_func_index = module.start_func_idx;
    for (idx, func) in module.functions_into_iter_topo_sort()? {
        sink.push(builder.proc(func_index_to_label(idx, &func_names), func.locals().len()));
        compile_function(func, config, &mut sink, &func_names)?;
    }
    sink.push(builder.begin());
    sink.push(builder.exec(func_index_to_label(start_func_index, &func_names)));
    sink.push(builder.end());
    Ok(sink)
}

pub fn compile_function(
    func: Func,
    config: &MidenTargetConfig,
    sink: &mut InstBuffer,
    func_names: &HashMap<FuncIndex, String>,
) -> Result<(), MidenError> {
    let mut iter = func.instructions_into_iter();
    let res = emit_inst(&mut iter, config, sink, func_names);
    if let Err(e) = res {
        return Err(e.into());
    }
    Ok(())
}

#[allow(clippy::unwrap_used)]
#[allow(unused_variables)]
#[cfg(test)]
mod tests {

    use super::*;
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
        use c2zk_ir_transform::miden::WasmToMidenLoweringPass;
        use ozk_frontend_wasm::WasmFrontendConfig;
        use pliron::context::Ptr;
        use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
        use pliron::linked_list::ContainsLinkedList;
        use pliron::op::Op;
        use pliron::operation::Operation;
        use pliron::pass::Pass;
        use pliron::with_context::AttachContext;

        let source = wat::parse_str(input).unwrap();
        let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
        let mut ctx = setup_context_dialects();
        let wasm_module_op = translate(&mut ctx, &source, frontend).unwrap();
        let wrapper_module = builtin::ops::ModuleOp::new(&mut ctx, "wrapper");
        wasm_module_op
            .get_operation()
            .insert_at_back(wrapper_module.get_body(&ctx, 0), &ctx);
        let triton_target_config = MidenTargetConfig::default();
        let pass = WasmToMidenLoweringPass::default();
        pass.run_on_operation(&mut ctx, wrapper_module.get_operation())
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
