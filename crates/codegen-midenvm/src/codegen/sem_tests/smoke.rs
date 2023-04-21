use super::check_miden;
use expect_test::expect;

#[test]
fn test_smoke() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![2];
    check_miden(
        r#"
(module 
    (start $main)
    (func $main 
        i32.const 1
        i32.const 1
        i32.add
        return)
)"#
        .to_string(),
        input,
        secret_input,
        expected_output,
        expect![[r#"
            proc.main.0
            push.1
            push.1
            add
            end

            proc.load_pub_outputs_on_stack.0
            end

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

            proc.save_pub_inputs.1
            push.2147483647
            loc_store.0
            sdepth
            push.16
            neq
            while.true
            loc_load.0
            swap.1
            swap.1
            mem_store
            loc_load.0
            push.1
            sub
            push.0
            exec.globals_set
            sdepth
            push.16
            neq
            end

            end

            proc.start_with_miden_io_persistent.0
            exec.save_pub_inputs
            exec.main
            exec.load_pub_outputs_on_stack
            end

            begin
            exec.start_with_miden_io_persistent
            end
        "#]],
    );
}
