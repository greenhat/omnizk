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
    for (idx, func) in module.functions_into_iter() {
        sink.push(builder.proc(func_index_to_label(idx, &func_names)));
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
#[cfg(test)]
mod tests {

    use super::*;
    use expect_test::expect;

    #[cfg(test)]
    fn check(input: &str, expected_tree: expect_test::Expect) {
        use c2zk_frontend::translate;
        use c2zk_frontend::FrontendConfig;
        use c2zk_frontend::WasmFrontendConfig;
        use c2zk_ir::pass::run_ir_passes;

        let source = wat::parse_str(input).unwrap();
        let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
        let mut module = translate(&source, frontend).unwrap();
        let triton_target_config = MidenTargetConfig::default();
        run_ir_passes(&mut module, &triton_target_config.ir_passes);
        let triton_target_config = MidenTargetConfig::default();
        dbg!(&module);
        let inst_buf = compile_module(module, &triton_target_config).unwrap();
        let out_source = inst_buf.pretty_print();
        expected_tree.assert_eq(&out_source);
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
                proc.f1
                push.1
                end

                proc.save_pub_inputs
                sdepth
                while.true
                sdepth
                push.2147483647
                dup.0
                swap.3
                swap.1
                mem_store
                push.-8
                add
                push.0
                exec.globals_set
                push.-1
                add
                end

                proc.globals_set
                push.-4
                mul
                push.2147467263
                add
                swap.1
                swap.1
                mem_store
                end

                proc.globals_get
                push.-4
                mul
                push.2147467263
                add
                mem_load
                end

                proc.load_pub_outputs_on_stack
                push.1
                exec.globals_get
                push.2147483647
                sub
                while.true
                push.1
                exec.globals_get
                dup.0
                mem_load
                push.8
                add
                dup.0
                push.1
                exec.globals_set
                push.2147483647
                sub
                end

                proc.start_with_miden_io_persistent
                exec.save_pub_inputs
                exec.f1
                exec.load_pub_outputs_on_stack
                end

                begin
                exec.start_with_miden_io_persistent
                end
            "#]],
        );
    }
}
