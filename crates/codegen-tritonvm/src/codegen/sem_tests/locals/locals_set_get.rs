use expect_test::expect;

use crate::codegen::sem_tests::check_wat;

#[test]
fn test_locals_set_get() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![9];
    check_wat(
        r#"
(module 
    (type (;0;) (func (result i64)))
    (type (;1;) (func (param i64)))
    (type (;2;) (func))
    (import "env" "c2zk_stdlib_pub_input" (func $c2zk_stdlib_pub_input (;0;) (type 0)))
    (import "env" "c2zk_stdlib_pub_output" (func $c2zk_stdlib_pub_output (;1;) (type 1)))
    (import "env" "c2zk_stdlib_secret_input" (func $c2zk_stdlib_secret_input (;2;) (type 0)))
    (export "main" (func $main))
    (start $main)
    (func $main (local i64)
        i64.const 9
        local.set 0
        local.get 0
        call $c2zk_stdlib_pub_output
        return)
)"#,
        input,
        secret_input,
        expected_output,
        expect![[r#"
            call main
            halt
            globals_set:
            push -4
            mul
            push 00000000002147482623
            add
            swap 1
            write_mem
            pop
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
            write_io
            push 0
            call globals_get
            push 4
            add
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
            main:
            call init_mem_for_locals
            push 0
            call globals_get
            push -4
            add
            push 0
            call globals_set
            push 9
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
            call c2zk_stdlib_pub_output
            push 0
            call globals_get
            push 4
            add
            push 0
            call globals_set
            return
            push 0
            call globals_get
            push 4
            add
            push 0
            call globals_set
            return
            init_mem_for_locals:
            push 00000000002147483647
            push 0
            call globals_set
            return"#]],
    );
}
