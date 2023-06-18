//! Semantic equivalence tests for the MidenVM codegen.

#![allow(clippy::unwrap_used)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::ops::RangeFrom;

use miden_assembly::Assembler;
use miden_processor::math::Felt;
use miden_processor::AdviceInputs;
use miden_processor::MemAdviceProvider;
use miden_processor::StackInputs;
use miden_processor::VmState;
use miden_processor::VmStateIterator;
use miden_stdlib::StdLibrary;
use ozk_codegen_midenvm::MidenTargetConfig;
use ozk_compiler::compile;
use ozk_frontend_wasm::WasmFrontendConfig;
use wasmtime::*;
use winter_math::StarkField;

pub fn check_wasm(
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

// fn compile(
//     source: &[u8],
//     frontend_config: &WasmFrontendConfig,
//     target_config: &MidenTargetConfig,
// ) -> Vec<u8> {
//     let mut ctx = Context::new();
//     frontend_config.register(&mut ctx);
//     target_config.register(&mut ctx);
//     let module = translate_module(&mut ctx, source).unwrap();
//     target_config
//         .pass_manager
//         .run(&mut ctx, module.get_operation())
//         .unwrap();
//     let target = MidenTarget::new(target_config);
//     let code = target.compile_module(module).unwrap();
//     code
// }

#[allow(unreachable_code)]
pub fn check_miden(
    source: String,
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_miden: expect_test::Expect,
) {
    let frontend_config = WasmFrontendConfig::default();
    let target_config = MidenTargetConfig::default();
    let wasm = wat::parse_str(source).unwrap();
    let miden_prog = compile(&wasm, frontend_config.into(), target_config.into()).unwrap();
    // let module = translate(&mut ctx, &wasm, frontend_config).unwrap();
    // run_ir_passes(&mut module, &target_config.ir_passes);
    // let inst_buf = compile_prog(module, &target_config).unwrap();
    // todo!("compile_module");
    // let inst_buf: InstBuffer = InstBuffer::new(&target_config);
    // let out_source = inst_buf.pretty_print();
    // expected_miden.assert_eq(&out_source);
    // let program = inst_buf.pretty_print();
    let program = String::from_utf8(miden_prog).unwrap();

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

pub fn check_wat(
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
