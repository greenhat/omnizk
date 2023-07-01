use crate::sem_tests::check_miden;

use expect_test::expect;
use sem_tests::check_ir;

mod sem_tests;

#[test]
fn test_smoke_ir() {
    check_ir(
        r#"
(module
    (start $f1)
    (func $f1
        i32.const 1
        return)
)"#,
        expect![[r#"
            miden.program {
              block_4_0():
                miden.proc @ozk_miden_main_proc {
                  entry():
                    miden.exec f1
                }
                miden.proc @f1 {
                  entry():
                    miden.constant 1: felt
                }
            }"#]],
    );
}

#[test]
fn test_smoke_add_wo_imports() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![3];
    check_miden(
        r#"
(module
    (start $main)
    (func $main
        i32.const 1
        i32.const 2
        i32.add
        return)
)"#,
        input,
        secret_input,
        expected_output,
        expect![[r#"
            proc.main.0
            push.1
            push.2
            add
            end

            begin
            exec.main
            end
        "#]],
    );
}
