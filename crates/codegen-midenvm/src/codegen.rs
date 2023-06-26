mod inst_buf;

pub use inst_buf::InstBuffer;
mod emit;
pub use emit::*;
mod miden_inst;
pub use miden_inst::*;
use ozk_miden_dialect::ops::*;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::op_interfaces::get_callees_syms;
use pliron::dialects::builtin::op_interfaces::SymbolOpInterface;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::with_context::AttachContext;
use thiserror::Error;
use topological_sort::TopologicalSort;

use crate::MidenError;
use crate::MidenTargetConfig;

pub fn emit_prog(
    ctx: &Context,
    op: Ptr<Operation>,
    target_config: &MidenTargetConfig,
) -> Result<InstBuffer, MidenError> {
    if let Some(prog_op) = op.deref(ctx).get_op(ctx).downcast_ref::<ProgramOp>() {
        todo!("compile miden program");
    } else {
        Err(MidenError::InvalidInst(format!(
            "expected ProgramOp, got {:?}",
            op.with_ctx(ctx).to_string()
        )))
    }
}

#[derive(Debug, Error)]
pub enum TopoSortError {
    #[error("Cycle in function dependencies: {0:?}")]
    Cycle(TopologicalSort<String>),
}

pub fn topo_sort_procedures(
    ctx: &Context,
    procedures: impl Iterator<Item = ProcOp>,
) -> Result<impl Iterator<Item = String>, TopoSortError> {
    let mut topo_sort = TopologicalSort::new();

    for proc in procedures {
        let proc_name = proc.get_symbol_name(ctx);
        topo_sort.insert(proc_name.clone());
        for dep in get_callees_syms(ctx, proc.get_operation()) {
            topo_sort.add_dependency(dep, proc_name.clone());
        }
    }
    let mut sorted = Vec::new();
    while !topo_sort.is_empty() {
        let mut func_indices = topo_sort.pop_all();
        if func_indices.is_empty() {
            return Err(TopoSortError::Cycle(topo_sort));
        }
        func_indices.sort();
        sorted.append(&mut func_indices);
    }
    Ok(sorted.into_iter())
}

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
                  block_4_0():
                    miden.proc @ozk_miden_main_proc {
                      entry():
                        miden.call f1
                    }
                    miden.proc @f1 {
                      entry():
                        miden.constant 1: felt
                    }
                }"#]],
        );
    }
}
