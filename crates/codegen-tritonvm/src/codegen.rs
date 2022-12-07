use c2zk_codegen_shared::CodegenError;
use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::Module;

mod inst_buf;
pub use inst_buf::InstBuffer;
mod emit;
pub use emit::emit_inst;

use triton_vm::instruction::AnInstruction;

use crate::TritonTargetConfig;

use self::emit::func_index_to_label;

pub fn compile_module(
    module: &Module,
    config: &TritonTargetConfig,
) -> Result<InstBuffer, CodegenError> {
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
) -> Result<(), CodegenError> {
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
        let inst_buf = compile_module(&module, &TritonTargetConfig::default()).unwrap();
        let out_source = inst_buf.pretty_print();
        expected_tree.assert_eq(&out_source);
        let program = inst_buf.program();
        let (_trace, _out, err) = program.run(vec![], vec![]);
        dbg!(&err);
        assert!(err.is_none());
    }

    fn check_wasm(
        source: &[u8],
        input: Vec<u64>,
        expected_output: Vec<u64>,
        expected_wat: expect_test::Expect,
        expected_triton: expect_test::Expect,
    ) {
        use c2zk_frontend::translate;
        use c2zk_frontend::FrontendConfig;
        use c2zk_frontend::WasmFrontendConfig;

        let wat = wasmprinter::print_bytes(source).unwrap();
        expected_wat.assert_eq(&wat);
        let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
        let module = translate(source, frontend).unwrap();
        let inst_buf = compile_module(&module, &TritonTargetConfig::default()).unwrap();
        let out_source = inst_buf.pretty_print();
        expected_triton.assert_eq(&out_source);
        let program = inst_buf.program();
        let input = input.into_iter().map(Into::into).collect();
        let (_trace, out, err) = program.run(input, vec![]);
        dbg!(&err);
        assert!(err.is_none());
        assert_eq!(
            out.into_iter().map(|b| b.into()).collect::<Vec<u64>>(),
            expected_output
        );
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

    #[test]
    fn test_from_rust() {
        let input = vec![11, 7];
        let expected_output = vec![18];
        let native_output = c2zk_rust_wasm_tests_helper::wrap_main_with_io(
            &c2zk_rust_wasm_tests_bundle1::main,
        )(input.clone());
        assert_eq!(native_output, expected_output);
        let wasm_bytes = c2zk_rust_wasm_tests_helper::compile_rust_wasm_tests_bundle1_bin("main");
        // TODO: extract the wat expected into a separate file (think of a larger code examples)
        check_wasm(
            &wasm_bytes,
            input,
            expected_output,
            expect![[r#"
                (module
                  (type (;0;) (func (result i64)))
                  (type (;1;) (func (param i64)))
                  (type (;2;) (func))
                  (type (;3;) (func (param i64 i64) (result i64)))
                  (import "env" "c2zk_stdlib_pub_input" (func $c2zk_stdlib_pub_input (;0;) (type 0)))
                  (import "env" "c2zk_stdlib_pub_output" (func $c2zk_stdlib_pub_output (;1;) (type 1)))
                  (func $__main (;2;) (type 2)
                    call $_ZN28c2zk_rust_wasm_tests_bundle14main17h8dbeeb4f115b4d38E
                  )
                  (func $_ZN28c2zk_rust_wasm_tests_bundle13add17hb70bbe9cd7dead81E (;3;) (type 3) (param i64 i64) (result i64)
                    local.get 1
                    local.get 0
                    i64.add
                  )
                  (func $_ZN28c2zk_rust_wasm_tests_bundle14main17h8dbeeb4f115b4d38E (;4;) (type 2)
                    call $_ZN11c2zk_stdlib9pub_input17h0dee22ae2ec5e4e8E
                    call $_ZN11c2zk_stdlib9pub_input17h0dee22ae2ec5e4e8E
                    call $_ZN28c2zk_rust_wasm_tests_bundle13add17hb70bbe9cd7dead81E
                    call $_ZN11c2zk_stdlib10pub_output17h07b1e1ebe272f489E
                  )
                  (func $_ZN11c2zk_stdlib9pub_input17h0dee22ae2ec5e4e8E (;5;) (type 0) (result i64)
                    call $c2zk_stdlib_pub_input
                  )
                  (func $_ZN11c2zk_stdlib10pub_output17h07b1e1ebe272f489E (;6;) (type 1) (param i64)
                    local.get 0
                    call $c2zk_stdlib_pub_output
                  )
                  (memory (;0;) 16)
                  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
                  (global (;1;) i32 i32.const 1048576)
                  (global (;2;) i32 i32.const 1048576)
                  (export "memory" (memory 0))
                  (export "__main" (func $__main))
                  (export "__data_end" (global 1))
                  (export "__heap_base" (global 2))
                )"#]],
            expect![[r#"
                call f2
                halt
                f0:
                read_io
                return
                f1:
                write_io
                return
                f2:
                call f4
                return
                f3:
                add
                return
                f4:
                call f5
                call f5
                call f3
                call f6
                return
                f5:
                call f0
                return
                f6:
                call f1
                return"#]],
        )
    }
}
