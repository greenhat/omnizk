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
            valida.program {
              entry():
                valida.imm32 -20(fp) 0 0 0 28
                valida.jal -28(fp) 4 -28 0 0
                valida.sw 0 4(fp) -24(fp) 0 0
                valida.exit
              block_4_1():
                valida.func @main {
                  entry():
                    valida.imm32 -4(fp) 0 0 0 1
                    valida.imm32 -8(fp) 0 0 0 2
                    valida.add -4(fp) -8(fp) -4(fp) 0 0
                    valida.sw 0 4(fp) -4(fp) 0 0
                    valida.jalv -4(fp) 0(fp) 8(fp) 0 0
                }
            }"#]],
    );
}

#[test]
fn test_smoke_add() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = 7.into();
    check_valida(
        r#"
(module
    (start $main)
    (func $main
        i32.const 3
        i32.const 4
        i32.add
        return)
)"#
        .to_string(),
        input,
        secret_input,
        expected_output,
        expect![[r#"
            valida.program {
              entry():
                valida.imm32 -20(fp) 0 0 0 28
                valida.jal -28(fp) 4 -28 0 0
                valida.sw 0 4(fp) -24(fp) 0 0
                valida.exit
              block_4_1():
                valida.func @main {
                  entry():
                    valida.imm32 -4(fp) 0 0 0 3
                    valida.imm32 -8(fp) 0 0 0 4
                    valida.add -4(fp) -8(fp) -4(fp) 0 0
                    valida.sw 0 4(fp) -4(fp) 0 0
                    valida.jalv -4(fp) 0(fp) 8(fp) 0 0
                }
            }"#]],
    );
}

#[ignore]
#[test]
fn test_smoke_func_call() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = 7.into();
    check_valida(
        r#"
(module
    (start $main)
    (func $add (param i32 i32) (result i32)
        get_local 0
        get_local 1
        i32.add
        return)
    (func $main
        i32.const 3
        i32.const 4
        call $add
        return)
)"#
        .to_string(),
        input,
        secret_input,
        expected_output,
        expect![[r#"
            valida.program {
              entry():
                valida.imm32 -20(fp) 0 0 0 28
                valida.jal -28(fp) 4 -28 0 0
                valida.sw 0 4(fp) -24(fp) 0 0
                valida.exit
              block_4_1():
                valida.func @main {
                  entry():
                    valida.imm32 -4(fp) 0 0 0 3
                    valida.imm32 -8(fp) 0 0 0 4
                    valida.add -4(fp) -8(fp) -4(fp) 0 0
                    valida.sw 0 4(fp) -4(fp) 0 0
                    valida.jalv -4(fp) 0(fp) 8(fp) 0 0
                }
            }"#]],
    );
}
