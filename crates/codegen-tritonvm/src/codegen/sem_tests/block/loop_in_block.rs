use expect_test::expect;

use crate::codegen::sem_tests::check_wat;

#[test]
fn test_loop_in_block() {
    let input = vec![1, 1, 0];
    let secret_input = vec![];
    let expected_output = vec![9, 9, 9, 7, 6, 3];
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
    (func $main 
        block
            loop 
                i64.const 9
                call $c2zk_stdlib_pub_output
                call $c2zk_stdlib_pub_input
                i64.const 1
                i64.eq
                br_if 0
                i64.const 7
                call $c2zk_stdlib_pub_output
            end
            i64.const 6
            call $c2zk_stdlib_pub_output
            br 0
            i64.const 5
            call $c2zk_stdlib_pub_output
        end
        i64.const 3
        call $c2zk_stdlib_pub_output
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
            main:
            call init_mem_for_locals
            call main_l0_b0
            push 3
            call c2zk_stdlib_pub_output
            return
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
            return
            main_l0_b0:
            call main_l0_b0_l1_b0
            call next_br_propagation
            skiz
            return
            push 6
            call c2zk_stdlib_pub_output
            return
            push 5
            call c2zk_stdlib_pub_output
            return
            main_l0_b0_l1_b0:
            push 9
            call c2zk_stdlib_pub_output
            call c2zk_stdlib_pub_input
            push 1
            eq
            skiz
            recurse
            push 7
            call c2zk_stdlib_pub_output
            return
            next_br_propagation:
            push -2
            call globals_get
            dup0
            push 0
            eq
            skiz
            return
            push -1
            add
            dup0
            push -2
            call globals_set
            return"#]],
    );
}
