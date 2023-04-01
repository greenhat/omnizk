use expect_test::expect;

use crate::codegen::sem_tests::check_wasm;

#[test]
fn test_add() {
    let input = vec![11, 7];
    let secret_input = vec![3];
    let expected_output = vec![21];
    let native_output = c2zk_rust_wasm_tests_helper::wrap_main_with_io(
        &c2zk_rust_wasm_tests_bundle1::add::main,
    )(input.clone(), secret_input.clone());
    assert_eq!(native_output, expected_output);
    let wasm_bytes = c2zk_rust_wasm_tests_helper::compile_rust_wasm_tests_bundle1_bin("add");
    check_wasm(
        &wasm_bytes,
        input,
        secret_input,
        expected_output,
        expect![[r#"
            (module
              (type (;0;) (func (result i64)))
              (type (;1;) (func (param i64)))
              (type (;2;) (func))
              (type (;3;) (func (param i64 i64) (result i64)))
              (import "env" "c2zk_stdlib_pub_input" (func $c2zk_stdlib_pub_input (;0;) (type 0)))
              (import "env" "c2zk_stdlib_pub_output" (func $c2zk_stdlib_pub_output (;1;) (type 1)))
              (import "env" "c2zk_stdlib_secret_input" (func $c2zk_stdlib_secret_input (;2;) (type 0)))
              (func $__main (;3;) (type 2)
                call $_ZN28c2zk_rust_wasm_tests_bundle13add4main17hff36f7d230d1624bE
              )
              (func $_ZN28c2zk_rust_wasm_tests_bundle13add3add17hb997205e011a93e7E (;4;) (type 3) (param i64 i64) (result i64)
                local.get 1
                local.get 0
                i64.add
              )
              (func $_ZN28c2zk_rust_wasm_tests_bundle13add4main17hff36f7d230d1624bE (;5;) (type 2)
                call $_ZN11c2zk_stdlib9pub_input17hb25830b97987a2f3E
                call $_ZN11c2zk_stdlib9pub_input17hb25830b97987a2f3E
                call $_ZN28c2zk_rust_wasm_tests_bundle13add3add17hb997205e011a93e7E
                call $_ZN11c2zk_stdlib12secret_input17hf97665cc0b784b7dE
                call $_ZN28c2zk_rust_wasm_tests_bundle13add3add17hb997205e011a93e7E
                call $_ZN11c2zk_stdlib10pub_output17h9103d5edd28554d4E
              )
              (func $_ZN11c2zk_stdlib9pub_input17hb25830b97987a2f3E (;6;) (type 0) (result i64)
                call $c2zk_stdlib_pub_input
              )
              (func $_ZN11c2zk_stdlib10pub_output17h9103d5edd28554d4E (;7;) (type 1) (param i64)
                local.get 0
                call $c2zk_stdlib_pub_output
              )
              (func $_ZN11c2zk_stdlib12secret_input17hf97665cc0b784b7dE (;8;) (type 0) (result i64)
                call $c2zk_stdlib_secret_input
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
            call __main
            halt
            c2zk_stdlib_pub_input:
            read_io
            return
            c2zk_stdlib_pub_output:
            push -1
            call globals_get
            dup 0
            swap 2
            write_mem
            pop
            push -4
            add
            push -1
            call globals_set
            push -1
            call globals_get
            push 4
            add
            read_mem
            swap 1
            pop
            write_io
            push -1
            call globals_get
            push 4
            add
            push -1
            call globals_set
            return
            c2zk_stdlib_secret_input:
            divine
            return
            __main:
            call init_mem_for_locals
            call _ZN28c2zk_rust_wasm_tests_bundle13add4main17hc047b7630b61d543E
            return
            _ZN28c2zk_rust_wasm_tests_bundle13add3add17h8212e59c2be7580eE:
            push -1
            call globals_get
            dup 0
            swap 2
            write_mem
            pop
            push -4
            add
            dup 0
            swap 2
            write_mem
            pop
            push -4
            add
            push -1
            call globals_set
            push -1
            call globals_get
            push 4
            add
            read_mem
            swap 1
            pop
            push -1
            call globals_get
            push 8
            add
            read_mem
            swap 1
            pop
            add
            push -1
            call globals_get
            push 8
            add
            push -1
            call globals_set
            return
            _ZN28c2zk_rust_wasm_tests_bundle13add4main17hc047b7630b61d543E:
            call _ZN11c2zk_stdlib9pub_input17had951df5cd6b60c2E
            call _ZN11c2zk_stdlib9pub_input17had951df5cd6b60c2E
            call _ZN28c2zk_rust_wasm_tests_bundle13add3add17h8212e59c2be7580eE
            call _ZN11c2zk_stdlib12secret_input17h461ee9797344b4dcE
            call _ZN28c2zk_rust_wasm_tests_bundle13add3add17h8212e59c2be7580eE
            call _ZN11c2zk_stdlib10pub_output17h88275f26ad69ad8bE
            return
            _ZN11c2zk_stdlib9pub_input17had951df5cd6b60c2E:
            call c2zk_stdlib_pub_input
            return
            _ZN11c2zk_stdlib10pub_output17h88275f26ad69ad8bE:
            push -1
            call globals_get
            dup 0
            swap 2
            write_mem
            pop
            push -4
            add
            push -1
            call globals_set
            push -1
            call globals_get
            push 4
            add
            read_mem
            swap 1
            pop
            call c2zk_stdlib_pub_output
            push -1
            call globals_get
            push 4
            add
            push -1
            call globals_set
            return
            _ZN11c2zk_stdlib12secret_input17h461ee9797344b4dcE:
            call c2zk_stdlib_secret_input
            return
            init_mem_for_locals:
            push 00000000002147483635
            push -1
            call globals_set
            return
            globals_get:
            push 4
            mul
            push 00000000002147483647
            add
            read_mem
            swap 1
            pop
            return
            globals_set:
            push 4
            mul
            push 00000000002147483647
            add
            swap 1
            write_mem
            pop
            return"#]],
    )
}
