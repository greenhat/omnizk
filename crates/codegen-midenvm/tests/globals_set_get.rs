use expect_test::expect;
use sem_tests::check_miden;

mod sem_tests;

#[test]
fn test_globals_set_get() {
    let input = vec![];

    let secret_input = vec![];
    let expected_output = vec![9];
    check_miden(
        r#"
(module
    (type (;2;) (func))
    (global $MyGlobalVal (mut i32) i32.const 42)
    (export "main" (func $main))
    (start $main)
    (func $main
        i32.const 9
        global.set $MyGlobalVal
        global.get $MyGlobalVal
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
            dup 0
            swap 2
            write_mem
            pop
            push -4
            add
            push -1
            call globals_set
            push -1
            call globals_get
            push 4
            add
            read_mem
            swap 1
            pop
            write_io
            push -1
            call globals_get
            push 4
            add
            push -1
            call globals_set
            return
            c2zk_stdlib_secret_input:
            divine
            return
            main:
            call init_mem_for_locals
            push -1
            call globals_get
            push -4
            add
            push -1
            call globals_set
            push 9
            push -1
            call globals_get
            swap 1
            swap 1
            push 4
            add
            swap 1
            write_mem
            pop
            push -1
            call globals_get
            push 4
            add
            read_mem
            swap 1
            pop
            call c2zk_stdlib_pub_output
            push -1
            call globals_get
            push 4
            add
            push -1
            call globals_set
            return
            push -1
            call globals_get
            push 4
            add
            push -1
            call globals_set
            return
            init_mem_for_locals:
            push 00000000002147483635
            push -1
            call globals_set
            return
            globals_get:
            push 4
            mul
            push 00000000002147483647
            add
            read_mem
            swap 1
            pop
            return
            globals_set:
            push 4
            mul
            push 00000000002147483647
            add
            swap 1
            write_mem
            pop
            return"#]],
    );
}
