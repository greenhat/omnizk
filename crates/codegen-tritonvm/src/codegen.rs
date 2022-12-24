use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::Module;

mod inst_buf;
pub use inst_buf::InstBuffer;
mod emit;
pub use emit::emit_inst;

#[cfg(test)]
mod sem_tests;

use triton_vm::instruction::AnInstruction;

use crate::TritonError;
use crate::TritonTargetConfig;

use self::emit::func_index_to_label;

pub fn compile_module(
    module: Module,
    config: &TritonTargetConfig,
) -> Result<InstBuffer, TritonError> {
    let mut sink = InstBuffer::new(config);
    sink.push(AnInstruction::Call(func_index_to_label(
        module.start_func_idx,
    )));
    sink.push(AnInstruction::Halt);
    for (idx, func) in module.functions().iter().enumerate() {
        let idx = FuncIndex::from(idx as u32);
        // TODO: use the original function name as label?
        sink.push_label(func_index_to_label(idx));
        compile_function(func, config, &mut sink)?;
    }
    Ok(sink)
}

pub fn compile_function(
    func: &Func,
    config: &TritonTargetConfig,
    sink: &mut InstBuffer,
) -> Result<(), TritonError> {
    for ins in func.instructions() {
        emit_inst(ins, config, sink)?;
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

        let source = wat::parse_str(input).unwrap();
        let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
        let module = translate(&source, frontend).unwrap();
        let inst_buf = compile_module(module, &TritonTargetConfig::default()).unwrap();
        let out_source = inst_buf.pretty_print();
        expected_tree.assert_eq(&out_source);
        let program = inst_buf.program();
        let (_trace, _out, err) = program.run(vec![], vec![]);
        dbg!(&err);
        assert!(err.is_none());
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
                call f0
                halt
                f0:
                push 1
                return
                return"#]],
        );
    }

    #[test]
    fn test_func_call() {
        check(
            r#"
(module 
    (start $main)
    (func $add (param i32 i32) (result i32)
        get_local 0
        get_local 1
        i32.add
        return)
    (func $main
        i32.const 1
        i32.const 2
        call $add
        return)
)"#,
            expect![[r#"
                call f1
                halt
                f0:
                add
                return
                return
                f1:
                push 1
                push 2
                call f0
                return
                return"#]],
        );
    }
}
