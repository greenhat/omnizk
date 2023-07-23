use expect_test::expect;

use crate::codegen::sem_tests::check_wasm;

#[test]
fn test_add() {
    let input = vec![11, 7];
    let secret_input = vec![3];
    let expected_output = vec![21];
    let native_output = ozk_rust_wasm_tests_helper::wrap_main_with_io(
        &ozk_rust_wasm_tests_add::add::main_add,
    )(input.clone(), secret_input.clone());
    assert_eq!(native_output, expected_output);
    let wasm_bytes = ozk_rust_wasm_tests_helper::compile_rust_wasm_tests("add-bin", "add");
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
              (import "env" "ozk_stdlib_pub_input" (func $ozk_stdlib_pub_input (;0;) (type 0)))
              (import "env" "ozk_stdlib_pub_output" (func $ozk_stdlib_pub_output (;1;) (type 1)))
              (import "env" "ozk_stdlib_secret_input" (func $ozk_stdlib_secret_input (;2;) (type 0)))
              (func $__main (;3;) (type 2)
                call $main_add
              )
              (func $add (;4;) (type 3) (param i64 i64) (result i64)
                local.get 1
                local.get 0
                i64.add
              )
              (func $main_add (;5;) (type 2)
                call $pub_input
                call $pub_input
                call $add
                call $secret_input
                call $add
                call $pub_output
              )
              (func $pub_input (;6;) (type 0) (result i64)
                call $ozk_stdlib_pub_input
              )
              (func $pub_output (;7;) (type 1) (param i64)
                local.get 0
                call $ozk_stdlib_pub_output
              )
              (func $secret_input (;8;) (type 0) (result i64)
                call $ozk_stdlib_secret_input
              )
              (memory (;0;) 16)
              (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
              (global (;1;) i32 i32.const 1048576)
              (global (;2;) i32 i32.const 1048576)
              (export "memory" (memory 0))
              (export "__main" (func $__main))
              (export "main_add" (func $main_add))
              (export "add" (func $add))
              (export "pub_input" (func $pub_input))
              (export "secret_input" (func $secret_input))
              (export "pub_output" (func $pub_output))
              (export "__data_end" (global 1))
              (export "__heap_base" (global 2))
            )"#]],
        expect![[r#"
            call __main
            halt
            ozk_stdlib_pub_input:
            read_io
            return
            ozk_stdlib_pub_output:
            push 0
            call globals_get
            push -4
            add
            push 0
            call globals_set
            push 0
            call globals_get
            swap 1
            write_mem
            pop
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
            ozk_stdlib_secret_input:
            divine
            return
            __main:
            call init_mem_for_locals
            call main_add
            return
            add:
            push -1
            call globals_get
            push -8
            add
            push 0
            call globals_set
            push 0
            call globals_get
            swap 1
            swap 1
            push 4
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            swap 1
            write_mem
            pop
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
            main_add:
            call pub_input
            call pub_input
            call add
            call secret_input
            call add
            call pub_output
            return
            pub_input:
            call ozk_stdlib_pub_input
            return
            pub_output:
            push -1
            call globals_get
            push -4
            add
            push 0
            call globals_set
            push 0
            call globals_get
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            read_mem
            swap 1
            pop
            call ozk_stdlib_pub_output
            push 0
            call globals_get
            push 4
            add
            push 0
            call globals_set
            return
            secret_input:
            call ozk_stdlib_secret_input
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
