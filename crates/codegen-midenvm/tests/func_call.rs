use expect_test::expect;
use sem_tests::check_ir;
use sem_tests::check_miden;

mod sem_tests;
use crate::sem_tests::check_wat;

#[test]
fn test_func_call_no_args() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![3];
    check_miden(
        r#"
(module
    (type (;0;) (func (result i32)))
    (type (;2;) (func))
    (export "main" (func $main))
    (start $main)
    (func $get (result i32)
        i32.const 1
        i32.const 2
        i32.add
        return)
    (func $main
        call $get
        return)
)"#,
        input,
        secret_input,
        expected_output,
        expect![[r#"
            proc.get.0
            push.1
            push.2
            add
            end

            proc.main.0
            exec.get
            end

            begin
            exec.main
            end
        "#]],
    );
}

#[test]
fn test_ir_func_call_w_args() {
    check_ir(
        r#"
(module
    (type (;0;) (func (result i32)))
    (type (;2;) (func))
    (export "main" (func $main))
    (start $main)
    (func $add (param i32 i32) (result i32)
        get_local 0
        get_local 1
        i32.add
        return)
    (func $main
        i32.const 1
        i32.const 2
        call $add
        return)
)"#,
        expect![[r#"
            miden.program {
              block_5_0():
                miden.proc @ozk_miden_main_proc {
                  entry():
                    miden.exec main
                }
                miden.proc @add {
                  entry():
                    wasm.local.set 0x0: ui32
                    wasm.local.set 0x1: ui32
                    wasm.local.get 0
                    wasm.local.get 1
                    miden.add
                }
                miden.proc @main {
                  entry():
                    miden.constant 1: felt
                    miden.constant 2: felt
                    miden.exec add
                }
            }"#]],
    );
}

#[ignore]
#[test]
fn test_func_call_w_args() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![3];
    check_miden(
        r#"
(module
    (type (;0;) (func (result i32)))
    (type (;2;) (func))
    (export "main" (func $main))
    (start $main)
    (func $add (param i32 i32) (result i32)
        get_local 0
        get_local 1
        i32.add
        return)
    (func $main
        i32.const 1
        i32.const 2
        call $add
        return)
)"#,
        input,
        secret_input,
        expected_output,
        expect![[r#"
            proc.get.0
            push.1
            push.2
            add
            end

            proc.main.0
            exec.get
            end

            begin
            exec.main
            end
        "#]],
    );
}

#[ignore]
#[test]
fn test_func_call() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![3];
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
    (func $add (param i64 i64) (result i64)
        get_local 0
        get_local 1
        i64.add
        return)
    (func $main
        i64.const 1
        i64.const 2
        call $add
        call $ozk_stdlib_pub_output
        return)
)"#,
        input,
        secret_input,
        expected_output,
        expect![[r#"
            proc.add.2
            loc_store.0
            loc_store.1
            loc_load.0
            loc_load.1
            add
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

            proc.ozk_stdlib_pub_output.1
            loc_store.0
            loc_load.0
            exec.omni_miden_pub_output
            end

            proc.main.0
            push.1
            push.2
            exec.add
            exec.ozk_stdlib_pub_output
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
