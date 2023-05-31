//! Semantic equivalence tests for the TritonVM codegen.

#![allow(clippy::unwrap_used)]

mod add;
mod block;
mod fib;
mod func_call;
mod locals;

use std::collections::HashMap;

use c2zk_frontend::translate_old;
use c2zk_ir::pass::run_ir_passes;
use ozk_frontend_wasm::WasmFrontendConfig;
use triton_vm::op_stack::OpStack;
use triton_vm::vm::VMState;
use twenty_first::shared_math::b_field_element::BFieldElement;
use wasmtime::*;

use crate::compile_module;
use crate::TritonTargetConfig;

fn check_wasm(
    source: &[u8],
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_wat: expect_test::Expect,
    expected_triton: expect_test::Expect,
) {
    let wat = wasmprinter::print_bytes(source).unwrap();
    expected_wat.assert_eq(&wat);

    check_triton(
        source,
        input,
        secret_input,
        expected_output,
        expected_triton,
    );
}

fn check_triton(
    wasm: &[u8],
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_triton: expect_test::Expect,
) {
    use c2zk_frontend::FrontendConfig;

    let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
    let triton_target_config = TritonTargetConfig::default();
    let mut module = translate_old(wasm, frontend).unwrap();
    run_ir_passes(&mut module, &triton_target_config.ir_passes);
    let inst_buf = compile_module(module, &triton_target_config).unwrap();
    let out_source = inst_buf.pretty_print();
    expected_triton.assert_eq(&out_source);
    let program = inst_buf.program();
    let input = input.into_iter().map(Into::into).collect();
    let secret_input = secret_input.into_iter().map(Into::into).collect();
    let (trace, out, err) = triton_vm::vm::debug(&program, input, secret_input);

    pp_trace(&trace);

    dbg!(&err);
    assert!(err.is_none());
    assert_eq!(
        out.into_iter().map(|b| b.into()).collect::<Vec<u64>>(),
        expected_output
    );
    let stack = pretty_stack(&trace.last().unwrap().op_stack);
    let expected_stack: Vec<u64> = vec![0; 16];
    assert_eq!(stack, expected_stack);
}

fn pp_trace(_trace: &[VMState]) {
    // iterate over last n traces
    for state in _trace.iter() {
        //.rev().take(400).rev() {
        let s = format!(
            "{}: {}",
            &state.current_instruction().unwrap(),
            pretty_print_vec_horiz(&pretty_stack(&state.op_stack))
        );
        dbg!(s);
        let r = pretty_print_ram_horiz(&state.ram);
        dbg!(r);
    }
}

fn check_wat(
    source: &str,
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_triton: expect_test::Expect,
) {
    struct Io {
        input: Vec<u64>,
        secret_input: Vec<u64>,
        output: Vec<u64>,
    }

    let mut store = Store::new(
        &Engine::default(),
        Io {
            input: input.clone().into_iter().rev().collect(),
            secret_input: secret_input.clone().into_iter().rev().collect(),
            output: Vec::new(),
        },
    );

    let wasm = wat::parse_str(source).unwrap();
    let module = Module::from_binary(store.engine(), &wasm).unwrap();

    let c2zk_stdlib_pub_input = Func::wrap(&mut store, |mut caller: Caller<'_, Io>| {
        caller.data_mut().input.pop().unwrap()
    });
    let c2zk_stdlib_pub_output =
        Func::wrap(&mut store, |mut caller: Caller<'_, Io>, output: i64| {
            caller.data_mut().output.push(output as u64);
        });
    let c2zk_stdlib_secret_input = Func::wrap(&mut store, |mut caller: Caller<'_, Io>| {
        caller.data_mut().secret_input.pop().unwrap()
    });
    let imports = [
        c2zk_stdlib_pub_input.into(),
        c2zk_stdlib_pub_output.into(),
        c2zk_stdlib_secret_input.into(),
    ];
    let _ = Instance::new(&mut store, &module, &imports).unwrap();

    assert_eq!(store.data().output, expected_output);
    check_triton(&wasm, input, secret_input, expected_output, expected_triton);
}

fn pretty_print_ram_horiz(ram: &HashMap<BFieldElement, BFieldElement>) -> String {
    // TODO: sort by key (pointer)
    // ram.iter().map(|(k, v)| (k.into(), v.into())).collect()
    let mut s = String::new();
    for (k, v) in ram.iter() {
        s.push_str(&format!("{}:{} ", k.value(), v.value()));
    }
    s
}

fn pretty_stack(stack: &OpStack) -> Vec<u64> {
    stack
        .stack
        .iter()
        .map(|b| b.value())
        // .filter(|v| *v != 0)
        .rev()
        .collect::<Vec<u64>>()
}

fn pretty_print_vec_horiz<T: std::fmt::Display>(vec: &[T]) -> String {
    let mut s = String::new();
    for v in vec {
        s.push_str(&format!("{} ", v));
    }
    s
}
