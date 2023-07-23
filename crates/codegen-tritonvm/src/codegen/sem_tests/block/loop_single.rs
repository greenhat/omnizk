use expect_test::expect;

use crate::codegen::sem_tests::check_wat;

#[test]
fn test_one_loop() {
    let input = vec![1, 1, 0];
    let secret_input = vec![];
    let expected_output = vec![9, 9, 9];
    check_wat(
        r#"
(module
    (type (;0;) (func (result i64)))
    (type (;1;) (func (param i64)))
    (type (;2;) (func))
    (import "env" "ozk_stdlib_pub_input" (func $ozk_stdlib_pub_input (;0;) (type 0)))
    (import "env" "ozk_stdlib_pub_output" (func $ozk_stdlib_pub_output (;1;) (type 1)))
    (import "env" "ozk_stdlib_secret_input" (func $ozk_stdlib_secret_input (;2;) (type 0)))
    (export "main" (func $main))
    (start $main)
    (func $main
        loop
            i64.const 9
            call $ozk_stdlib_pub_output
            call $ozk_stdlib_pub_input
            i64.const 1
            i64.eq
            br_if 0
        end
        return)
)"#,
        input,
        secret_input,
        expected_output,
        expect![[r#"
            call main
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
            globals_set:
            push -4
            mul
            push 00000000002147482623
            add
            swap 1
            write_mem
            pop
            return
            main:
            call init_mem_for_locals
            call main_l0_b0
            return
            return
            init_mem_for_locals:
            push 00000000002147483647
            push 0
            call globals_set
            return
            main_l0_b0:
            push 9
            call ozk_stdlib_pub_output
            call ozk_stdlib_pub_input
            push 1
            eq
            skiz
            recurse
            return
            globals_get:
            push -4
            mul
            push 00000000002147482623
            add
            read_mem
            swap 1
            pop
            return"#]],
    );
}
