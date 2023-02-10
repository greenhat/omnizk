use expect_test::expect;

use super::check_wat;

#[test]
fn test_func_call() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![];
    let expected_stack = vec![3];
    check_wat(
        r#"
(module 
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
        expected_stack,
        expect![[r#"
            (module
              (type (;0;) (func (param i32 i32) (result i32)))
              (type (;1;) (func))
              (func $add (;0;) (type 0) (param i32 i32) (result i32)
                local.get 0
                local.get 1
                i32.add
                return
              )
              (func $main (;1;) (type 1)
                i32.const 1
                i32.const 2
                call $add
                return
              )
              (start $main)
            )"#]],
        expect![[r#"
                call main
                halt
                add:
                push -1
                call globals_get
                dup0
                swap2
                write_mem
                pop
                pop
                push -1
                add
                dup0
                swap2
                write_mem
                pop
                pop
                push -1
                add
                push -1
                call globals_set
                push -1
                call globals_get
                push 2
                add
                push 0
                read_mem
                swap1
                pop
                push -1
                call globals_get
                push 1
                add
                push 0
                read_mem
                swap1
                pop
                add
                push -1
                call globals_get
                push 2
                add
                push -1
                call globals_set
                return
                push -1
                call globals_get
                push 2
                add
                push -1
                call globals_set
                return
                main:
                call init_mem_for_locals
                push 1
                push 2
                call add
                return
                return
                init_mem_for_locals:
                push 00000000002147483643
                push -1
                call globals_set
                return
                globals_get:
                push 00000000002147483647
                add
                push 0
                read_mem
                swap1
                pop
                return
                globals_set:
                push 00000000002147483647
                add
                swap1
                write_mem
                pop
                pop
                return"#]],
    );
}
