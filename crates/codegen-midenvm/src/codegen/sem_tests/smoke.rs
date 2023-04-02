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
            begin
            exec.f1
            end"#]],
    );
}
