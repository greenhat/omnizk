use super::check_miden;
use expect_test::expect;

#[test]
fn test_pub_inputs() {
    // let input = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2];
    let input = vec![5, 7];
    // prepend with 16 zeroes
    // let input = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2];
    let secret_input = vec![];
    let expected_output = vec![7, 5];
    check_miden(
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
        call $c2zk_stdlib_pub_input
        call $c2zk_stdlib_pub_input
        return)
)"#
        .to_string(),
        input,
        secret_input,
        expected_output,
        expect![[r#"
            proc.globals_get.0
            push.18446744069414584317
            mul
            push.2147467263
            add
            mem_load
            end

            proc.globals_set.0
            push.18446744069414584317
            mul
            push.2147467263
            add
            swap.1
            swap.1
            mem_store
            end

            proc.save_pub_inputs.2
            push.2147483647
            loc_store.0
            sdepth
            loc_store.1
            push.1
            while.true
            dup.0
            neq.0
            if.true
            loc_load.0
            dup.0
            swap.2
            swap.1
            mem_store
            push.8
            sub
            loc_store.0
            else
            drop
            end

            loc_load.1
            push.1
            sub
            dup.0
            loc_store.1
            neq.0
            end

            loc_load.0
            push.0
            exec.globals_set
            end

            proc.omni_miden_pub_input.0
            push.0
            exec.globals_get
            push.8
            add
            dup.0
            mem_load
            swap.1
            push.0
            exec.globals_set
            end

            proc.init_pub_outputs.0
            push.2147483647
            push.1
            exec.globals_set
            end

            proc.omni_miden_pub_output.0
            push.1
            exec.globals_get
            dup.0
            swap.2
            swap.1
            mem_store
            push.8
            sub
            push.1
            exec.globals_set
            end

            proc.load_pub_outputs_on_stack.1
            push.2147483647
            push.1
            exec.globals_get
            dup.0
            loc_store.0
            sub
            neq.0
            while.true
            loc_load.0
            dup.0
            mem_load
            push.8
            add
            dup.0
            loc_store.0
            push.2147483647
            swap.1
            sub
            dup.0
            neq.0
            end

            end

            proc.c2zk_stdlib_pub_input.0
            exec.omni_miden_pub_input
            end

            proc.main.0
            exec.c2zk_stdlib_pub_input
            exec.c2zk_stdlib_pub_input
            end

            proc.start_with_miden_io_persistent.0
            exec.save_pub_inputs
            exec.init_pub_outputs
            exec.main
            exec.load_pub_outputs_on_stack
            end

            begin
            exec.start_with_miden_io_persistent
            end
        "#]],
    );
}
