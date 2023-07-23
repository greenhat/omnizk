use expect_test::expect;

mod sem_tests;
use crate::sem_tests::check_wat;

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

            proc.load_pub_outputs_on_stack.2
            push.2147483647
            dup.0
            loc_store.0
            push.1
            exec.globals_get
            dup.0
            loc_store.1
            sub
            neq.0
            while.true
            loc_load.0
            mem_load
            loc_load.0
            push.8
            sub
            dup.0
            loc_store.0
            loc_load.1
            sub
            neq.0
            end

            end

            proc.c2zk_stdlib_pub_output.1
            loc_store.0
            loc_load.0
            exec.omni_miden_pub_output
            end

            proc.main_l0_b0.0
            push.1
            push.2
            add
            exec.c2zk_stdlib_pub_output
            end

            proc.main.0
            exec.main_l0_b0
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
