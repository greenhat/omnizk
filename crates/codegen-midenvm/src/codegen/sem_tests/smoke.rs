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
    (start $f1)
    (func $f1 
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
            proc.f1
            push.1
            push.1
            add
            end

            proc.save_pub_inputs
            sdepth
            while.true
            sdepth
            push.2147483647
            dup.0
            swap.3
            swap.1
            mem_store
            push.18446744069414584313
            add
            push.0
            exec.globals_set
            push.18446744069414584320
            add
            end

            proc.globals_set
            push.18446744069414584317
            mul
            push.2147467263
            add
            swap.1
            swap.1
            mem_store
            end

            proc.globals_get
            push.18446744069414584317
            mul
            push.2147467263
            add
            mem_load
            end

            proc.load_pub_outputs_on_stack
            push.1
            exec.globals_get
            push.2147483647
            sub
            while.true
            push.1
            exec.globals_get
            dup.0
            mem_load
            push.8
            add
            dup.0
            push.1
            exec.globals_set
            push.2147483647
            sub
            end

            proc.start_with_miden_io_persistent
            exec.save_pub_inputs
            exec.f1
            exec.load_pub_outputs_on_stack
            end

            begin
            exec.start_with_miden_io_persistent
            end
        "#]],
    );
}
