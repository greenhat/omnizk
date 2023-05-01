//! Semantic equivalence tests for the MidenVM codegen.

#![allow(clippy::unwrap_used)]

// mod add;
mod pub_inputs;
mod pub_outputs;
mod smoke;

// mod block;
// mod fib;
// mod func_call;
// mod locals;

use std::ops::RangeFrom;

use c2zk_ir::pass::run_ir_passes;
use miden_assembly::Assembler;
use miden_processor::math::Felt;
use miden_processor::AdviceInputs;
use miden_processor::MemAdviceProvider;
use miden_processor::StackInputs;
use miden_processor::VmState;
use miden_processor::VmStateIterator;
use miden_stdlib::StdLibrary;
use wasmtime::*;
use winter_math::StarkField;

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
    dbg!(&program);
    // let trace = miden_processor::execute(&program, stack_inputs, adv_provider).unwrap();
    let e_iter = miden_processor::execute_iter(&program, stack_inputs, adv_provider);
    let vm_state = build_vm_state(e_iter, 0..);
    eprintln!(
        "{}",
        &vm_state
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
    // assert_eq!(0, 1);
    // let stack = pretty_stack(trace.stack_outputs().stack());
    let stack = pretty_stack_felt(&vm_state.last().unwrap().stack);
    // fill expected_output with zeros if it's shorter than stack
    let expected_output = expected_output
        .into_iter()
        .chain(std::iter::repeat(0))
        .take(stack.len())
        .collect::<Vec<_>>();
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

fn pretty_stack_felt(stack: &[Felt]) -> Vec<u64> {
    stack.iter().map(|x| x.as_int()).collect::<Vec<_>>()
}

/// This is a helper function to build a vector of [VmStatePartial] from a specified [VmStateIterator].
fn build_vm_state(vm_state_iterator: VmStateIterator, range: RangeFrom<usize>) -> Vec<VmState> {
    let mut vm_state = Vec::new();
    for (idx, state) in vm_state_iterator.enumerate() {
        if !range.contains(&idx) {
            continue;
        }
        vm_state.push(state.unwrap());
    }
    vm_state
}
