use expect_test::expect;

use crate::codegen::sem_tests::check_wat;

#[test]
fn test_locals_func_exit() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![5];
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
    (func $add (param i64 i64) (result i64)
        get_local 0
        get_local 1
        i64.add
        return)
    (func $main (local i64)
        i64.const 5
        local.set 0
        i64.const 9
        i64.const 7
        call $add
        local.get 0
        call $c2zk_stdlib_pub_output ;; should return first(0) local value
        return)
)"#,
        input,
        secret_input,
        expected_output,
        expect![[r#"
            call main
            halt
            c2zk_stdlib_pub_input:
            read_io
            return
            c2zk_stdlib_pub_output:
            push -1
            call globals_get
            dup0
            swap2
            write_mem
            pop
            pop
            push -1
            add
            push -1
            call globals_set
            push -1
            call globals_get
            push 1
            add
            push 0
            read_mem
            swap1
            pop
            write_io
            push -1
            call globals_get
            push 1
            add
            push -1
            call globals_set
            return
            c2zk_stdlib_secret_input:
            divine
            return
            add:
            push -1
            call globals_get
            dup0
            swap2
            write_mem
            pop
            pop
            push -1
            add
            dup0
            swap2
            write_mem
            pop
            pop
            push -1
            add
            push -1
            call globals_set
            push -1
            call globals_get
            push 2
            add
            push 0
            read_mem
            swap1
            pop
            push -1
            call globals_get
            push 1
            add
            push 0
            read_mem
            swap1
            pop
            add
            push -1
            call globals_get
            push 2
            add
            push -1
            call globals_set
            return
            push -1
            call globals_get
            push 2
            add
            push -1
            call globals_set
            return
            main:
            call init_mem_for_locals
            push -1
            call globals_get
            push -1
            add
            push -1
            call globals_set
            push 5
            push -1
            call globals_get
            swap1
            swap1
            push 1
            add
            swap1
            write_mem
            pop
            pop
            push 9
            push 7
            call add
            push -1
            call globals_get
            push 1
            add
            push 0
            read_mem
            swap1
            pop
            call c2zk_stdlib_pub_output
            push -1
            call globals_get
            push 1
            add
            push -1
            call globals_set
            return
            push -1
            call globals_get
            push 1
            add
            push -1
            call globals_set
            return
            init_mem_for_locals:
            push 00000000002147483643
            push -1
            call globals_set
            return
            globals_get:
            push 00000000002147483647
            add
            push 0
            read_mem
            swap1
            pop
            return
            globals_set:
            push 00000000002147483647
            add
            swap1
            write_mem
            pop
            pop
            return"#]],
    );
}