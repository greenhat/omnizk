//! Semantic equivalence tests for the MidenVM codegen.

#![allow(clippy::unwrap_used)]

// mod add;
// mod smoke;

// mod block;
// mod fib;
// mod func_call;
// mod locals;

use c2zk_ir::pass::run_ir_passes;
use miden_assembly::Assembler;
use miden_processor::AdviceInputs;
use miden_processor::MemAdviceProvider;
use miden_processor::StackInputs;
use miden_stdlib::StdLibrary;
use wasmtime::*;

use crate::compile_module;
use crate::MidenTargetConfig;

fn check_wasm(
    source: &[u8],
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_wat: expect_test::Expect,
    expected_miden: expect_test::Expect,
) {
    let wat = wasmprinter::print_bytes(source).unwrap();
    expected_wat.assert_eq(&wat);

    check_miden(wat, input, secret_input, expected_output, expected_miden);
}

fn check_miden(
    source: String,
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_miden: expect_test::Expect,
) {
    use c2zk_frontend::translate;
    use c2zk_frontend::FrontendConfig;
    use c2zk_frontend::WasmFrontendConfig;

    let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
    let target_config = MidenTargetConfig::default();
    let wasm = wat::parse_str(source).unwrap();
    let mut module = translate(&wasm, frontend).unwrap();
    run_ir_passes(&mut module, &target_config.ir_passes);
    let inst_buf = compile_module(module, &target_config).unwrap();
    let out_source = inst_buf.pretty_print();
    expected_miden.assert_eq(&out_source);
    let program = inst_buf.pretty_print();

    let assembler = Assembler::default()
        .with_library(&StdLibrary::default())
        .unwrap();
    let program = assembler.compile(program).unwrap();
    let stack_inputs = StackInputs::try_from_values(input).unwrap();
    let adv_provider: MemAdviceProvider = AdviceInputs::default()
        .with_stack_values(secret_input)
        .unwrap()
        .into();
    let trace = miden_processor::execute(&program, stack_inputs, adv_provider).unwrap();
    let stack = pretty_stack(trace.stack_outputs().stack());
    assert_eq!(stack, expected_output);
}

fn check_wat(
    source: &str,
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_miden: expect_test::Expect,
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
    check_miden(
        source.to_string(),
        input,
        secret_input,
        expected_output,
        expected_miden,
    );
}

fn pretty_stack(stack: &[u64]) -> Vec<u64> {
    stack
        .iter()
        .copied()
        .filter(|x| *x != 0)
        .collect::<Vec<_>>()
}
