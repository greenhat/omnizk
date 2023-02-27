use expect_test::expect;

use crate::codegen::sem_tests::check_wat;

#[test]
fn test_one_block() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![3];
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
        block ;; label = @1
          i64.const 1
          i64.const 2
          i64.add
          call $c2zk_stdlib_pub_output
        end
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
            push -1 // Begin: propagate Br* in block (0)
            add
            skiz
            return // End: propagate Br* in block
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
            push 1
            push 2
            add
            call c2zk_stdlib_pub_output
            push 1 // Begin: extracted func prologue (0)
            return // End: extracted func prologue"#]],
    );
}
