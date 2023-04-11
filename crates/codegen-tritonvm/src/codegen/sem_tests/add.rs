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
                call $_ZN28c2zk_rust_wasm_tests_bundle13add4main17h9ee4833820ee571bE
              )
              (func $_ZN28c2zk_rust_wasm_tests_bundle13add3add17ha0201f3b48af1943E (;4;) (type 3) (param i64 i64) (result i64)
                local.get 1
                local.get 0
                i64.add
              )
              (func $_ZN28c2zk_rust_wasm_tests_bundle13add4main17h9ee4833820ee571bE (;5;) (type 2)
                call $_ZN11c2zk_stdlib9pub_input17h4dc54be4a473b24cE
                call $_ZN11c2zk_stdlib9pub_input17h4dc54be4a473b24cE
                call $_ZN28c2zk_rust_wasm_tests_bundle13add3add17ha0201f3b48af1943E
                call $_ZN11c2zk_stdlib12secret_input17he19f6aa9041d5ef2E
                call $_ZN28c2zk_rust_wasm_tests_bundle13add3add17ha0201f3b48af1943E
                call $_ZN11c2zk_stdlib10pub_output17h052bf0348afc7d69E
              )
              (func $_ZN11c2zk_stdlib9pub_input17h4dc54be4a473b24cE (;6;) (type 0) (result i64)
                call $c2zk_stdlib_pub_input
              )
              (func $_ZN11c2zk_stdlib10pub_output17h052bf0348afc7d69E (;7;) (type 1) (param i64)
                local.get 0
                call $c2zk_stdlib_pub_output
              )
              (func $_ZN11c2zk_stdlib12secret_input17he19f6aa9041d5ef2E (;8;) (type 0) (result i64)
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
            push 0
            call globals_get
            push -4
            add
            dup 0
            swap 2
            write_mem
            pop
            push 0
            call globals_set
            push 0
            call globals_get
            read_mem
            swap 1
            pop
            write_io
            push 0
            call globals_get
            push 4
            add
            push 0
            call globals_set
            return
            c2zk_stdlib_secret_input:
            divine
            return
            __main:
            call init_mem_for_locals
            call _ZN28c2zk_rust_wasm_tests_bundle13add4main17h9ee4833820ee571bE
            return
            _ZN28c2zk_rust_wasm_tests_bundle13add3add17ha0201f3b48af1943E:
            push 0
            call globals_get
            push -4
            add
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
            push 0
            call globals_set
            push 0
            call globals_get
            read_mem
            swap 1
            pop
            push 0
            call globals_get
            push 4
            add
            read_mem
            swap 1
            pop
            add
            push 0
            call globals_get
            push 8
            add
            push 0
            call globals_set
            return
            _ZN28c2zk_rust_wasm_tests_bundle13add4main17h9ee4833820ee571bE:
            call _ZN11c2zk_stdlib9pub_input17h4dc54be4a473b24cE
            call _ZN11c2zk_stdlib9pub_input17h4dc54be4a473b24cE
            call _ZN28c2zk_rust_wasm_tests_bundle13add3add17ha0201f3b48af1943E
            call _ZN11c2zk_stdlib12secret_input17he19f6aa9041d5ef2E
            call _ZN28c2zk_rust_wasm_tests_bundle13add3add17ha0201f3b48af1943E
            call _ZN11c2zk_stdlib10pub_output17h052bf0348afc7d69E
            return
            _ZN11c2zk_stdlib9pub_input17h4dc54be4a473b24cE:
            call c2zk_stdlib_pub_input
            return
            _ZN11c2zk_stdlib10pub_output17h052bf0348afc7d69E:
            push 0
            call globals_get
            push -4
            add
            dup 0
            swap 2
            write_mem
            pop
            push 0
            call globals_set
            push 0
            call globals_get
            read_mem
            swap 1
            pop
            call c2zk_stdlib_pub_output
            push 0
            call globals_get
            push 4
            add
            push 0
            call globals_set
            return
            _ZN11c2zk_stdlib12secret_input17he19f6aa9041d5ef2E:
            call c2zk_stdlib_secret_input
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
    )
}
