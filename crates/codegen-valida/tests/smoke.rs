mod sem_tests;
use crate::sem_tests::check_valida;
use sem_tests::check_ir;

use expect_test::expect;

#[test]
fn test_smoke_ir() {
    check_ir(
        r#"
(module
    (start $main)
    (func $main
        i32.const 1
        i32.const 2
        i32.add
        return)
)"#,
        expect![[r#"
            "#]],
    );
}

#[ignore]
#[test]
fn test_smoke_add_wo_imports() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![2];
    check_valida(
        r#"
(module
    (start $main)
    (func $main
        i32.const 1
        i32.const 2
        i32.add
        return)
)"#
        .to_string(),
        input,
        secret_input,
        expected_output,
        expect![[r#"
        "#]],
    );
}
