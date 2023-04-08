use c2zk_codegen_shared::func_index_to_label;
use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::Module;

mod inst_buf;
pub use inst_buf::InstBuffer;
mod emit;
pub use emit::emit_inst;
use triton_opcodes::instruction::AnInstruction;

#[cfg(test)]
mod sem_tests;

use crate::TritonError;
use crate::TritonTargetConfig;

pub fn compile_module(
    module: Module,
    config: &TritonTargetConfig,
) -> Result<InstBuffer, TritonError> {
    let mut sink = InstBuffer::new(config);
    let func_names = module.func_names();
    sink.push(AnInstruction::Call(func_index_to_label(
        module.start_func_idx,
        &func_names,
    )));
    sink.push(AnInstruction::Halt);
    for (idx, func) in module.into_functions().into_iter().enumerate() {
        let idx = FuncIndex::from(idx as u32);
        sink.push_label(func_index_to_label(idx, &func_names));
        compile_function(func, config, &mut sink, &func_names)?;
    }
    Ok(sink)
}

pub fn compile_function(
    func: Func,
    config: &TritonTargetConfig,
    sink: &mut InstBuffer,
    func_names: &[String],
) -> Result<(), TritonError> {
    for (idx, ins) in func.instructions().iter().enumerate() {
        if let Some(comment) = func.comments().get(&idx) {
            sink.push_comment_for_next_ins(comment.clone());
        } else {
        }
        let res = emit_inst(ins, config, sink, func_names);
        if let Err(e) = res {
            dbg!(&func);
            return Err(e);
        }
    }
    Ok(())
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::*;

    #[cfg(test)]
    fn check(input: &str, expected_tree: expect_test::Expect) {
        use c2zk_frontend::translate;
        use c2zk_frontend::FrontendConfig;
        use c2zk_frontend::WasmFrontendConfig;
        use c2zk_ir::pass::run_ir_passes;

        let source = wat::parse_str(input).unwrap();
        let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
        let mut module = translate(&source, frontend).unwrap();
        let triton_target_config = TritonTargetConfig::default();
        run_ir_passes(&mut module, &triton_target_config.ir_passes);
        let triton_target_config = TritonTargetConfig::default();
        let inst_buf = compile_module(module, &triton_target_config).unwrap();
        let out_source = inst_buf.pretty_print();
        expected_tree.assert_eq(&out_source);
    }

    #[test]
    fn test_start_section() {
        check(
            r#"
(module 
    (start $f1)
    (func $f1 
        i32.const 1
        return)
)"#,
            expect![[r#"
                call f1
                halt
                f1:
                call init_mem_for_locals
                push 1
                return
                return
                init_mem_for_locals:
                push 00000000002147483647
                push 0
                call globals_set
                return
                globals_get:
                push -4
                mul
                push 00000000002147482623
                add
                read_mem
                swap 1
                pop
                return
                globals_set:
                push -4
                mul
                push 00000000002147482623
                add
                swap 1
                write_mem
                pop
                return"#]],
        );
    }
}
