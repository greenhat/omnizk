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
                call $_ZN28c2zk_rust_wasm_tests_bundle13add4main17hab51675481e443caE
              )
              (func $_ZN28c2zk_rust_wasm_tests_bundle13add3add17hd10e69d3d128fc14E (;4;) (type 3) (param i64 i64) (result i64)
                local.get 1
                local.get 0
                i64.add
              )
              (func $_ZN28c2zk_rust_wasm_tests_bundle13add4main17hab51675481e443caE (;5;) (type 2)
                call $_ZN11c2zk_stdlib9pub_input17h060bd075f37c6b24E
                call $_ZN11c2zk_stdlib9pub_input17h060bd075f37c6b24E
                call $_ZN28c2zk_rust_wasm_tests_bundle13add3add17hd10e69d3d128fc14E
                call $_ZN11c2zk_stdlib12secret_input17hfc353234bd7a7fadE
                call $_ZN28c2zk_rust_wasm_tests_bundle13add3add17hd10e69d3d128fc14E
                call $_ZN11c2zk_stdlib10pub_output17hc744a302b8a83f64E
              )
              (func $_ZN11c2zk_stdlib9pub_input17h060bd075f37c6b24E (;6;) (type 0) (result i64)
                call $c2zk_stdlib_pub_input
              )
              (func $_ZN11c2zk_stdlib10pub_output17hc744a302b8a83f64E (;7;) (type 1) (param i64)
                local.get 0
                call $c2zk_stdlib_pub_output
              )
              (func $_ZN11c2zk_stdlib12secret_input17hfc353234bd7a7fadE (;8;) (type 0) (result i64)
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
            push 00000000002147483647
            push 0
            add
            read_mem
            push -2
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            read_io
            return
            c2zk_stdlib_pub_output:
            push 00000000002147483647
            push 0
            add
            read_mem
            push 0
            add
            write_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push -1
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push -2
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            write_io
            return
            c2zk_stdlib_secret_input:
            push 00000000002147483647
            push 0
            add
            read_mem
            push -2
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            divine
            return
            __main:
            call init_mem_for_locals
            push 00000000002147483647
            push 0
            add
            read_mem
            push -2
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            call _ZN28c2zk_rust_wasm_tests_bundle13add4main17hab51675481e443caE
            return
            _ZN28c2zk_rust_wasm_tests_bundle13add3add17hd10e69d3d128fc14E:
            push 00000000002147483647
            push 0
            add
            read_mem
            push 0
            add
            write_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push -1
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push 0
            add
            write_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push -1
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push -2
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push 1
            add
            read_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push 0
            add
            read_mem
            add
            return
            _ZN28c2zk_rust_wasm_tests_bundle13add4main17hab51675481e443caE:
            push 00000000002147483647
            push 0
            add
            read_mem
            push -2
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            call _ZN11c2zk_stdlib9pub_input17h060bd075f37c6b24E
            call _ZN11c2zk_stdlib9pub_input17h060bd075f37c6b24E
            call _ZN28c2zk_rust_wasm_tests_bundle13add3add17hd10e69d3d128fc14E
            call _ZN11c2zk_stdlib12secret_input17hfc353234bd7a7fadE
            call _ZN28c2zk_rust_wasm_tests_bundle13add3add17hd10e69d3d128fc14E
            call _ZN11c2zk_stdlib10pub_output17hc744a302b8a83f64E
            return
            _ZN11c2zk_stdlib9pub_input17h060bd075f37c6b24E:
            push 00000000002147483647
            push 0
            add
            read_mem
            push -2
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            call c2zk_stdlib_pub_input
            return
            _ZN11c2zk_stdlib10pub_output17hc744a302b8a83f64E:
            push 00000000002147483647
            push 0
            add
            read_mem
            push 0
            add
            write_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push -1
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push -2
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            push 00000000002147483647
            push 0
            add
            read_mem
            push 0
            add
            read_mem
            call c2zk_stdlib_pub_output
            return
            _ZN11c2zk_stdlib12secret_input17hfc353234bd7a7fadE:
            push 00000000002147483647
            push 0
            add
            read_mem
            push -2
            add
            push 00000000002147483647
            push 0
            add
            write_mem
            call c2zk_stdlib_secret_input
            return
            init_mem_for_locals:
            push 00000000002147483643
            push 00000000002147483647
            push 0
            add
            write_mem"#]],
    )
}
