//! Semantic equivalence tests for the Valida VM codegen.

#![allow(unused_imports)]
#![allow(clippy::unwrap_used)]
#![allow(unused_variables)]
#![allow(dead_code)]

use ozk_codegen_valida::emit_op;
use ozk_codegen_valida::ValidaInstrBuilder;
use ozk_codegen_valida::ValidaTargetConfig;
use ozk_frontend_wasm::WasmFrontendConfig;
use ozk_valida_dialect::ops::ProgramOp;
use ozk_wasm_dialect::ops::ModuleOp;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin;
use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::with_context::AttachContext;
use valida_basic::BasicMachine;
use valida_machine::Word;
use wasmtime::*;

pub fn check_ir(input: &str, expected_tree: expect_test::Expect) {
    let source = wat::parse_str(input).unwrap();
    let mut ctx = Context::default();
    let target_config = ValidaTargetConfig::default();
    let frontend_config = WasmFrontendConfig::default();
    frontend_config.register(&mut ctx);
    target_config.register(&mut ctx);
    let wasm_module_op =
        ozk_frontend_wasm::parse_module(&mut ctx, &source, &frontend_config).unwrap();
    let prog = run_conversion_passes(&mut ctx, wasm_module_op, &target_config);
    expected_tree.assert_eq(prog.with_ctx(&ctx).to_string().as_str());
}

fn run_conversion_passes(
    ctx: &mut Context,
    wasm_module: ModuleOp,
    target_config: &ValidaTargetConfig,
) -> ProgramOp {
    // we need to wrap the wasm in an op because passes cannot replace the root op
    let wrapper_module = builtin::ops::ModuleOp::new(ctx, "wrapper");
    wasm_module
        .get_operation()
        .insert_at_back(wrapper_module.get_body(ctx, 0), ctx);
    target_config
        .pass_manager
        .run(ctx, wrapper_module.get_operation())
        .unwrap();
    let inner_module = wrapper_module
        .get_body(ctx, 0)
        .deref(ctx)
        .iter(ctx)
        .collect::<Vec<Ptr<Operation>>>()
        .first()
        .cloned()
        .unwrap();
    *inner_module
        .deref(ctx)
        .get_op(ctx)
        .downcast::<ProgramOp>()
        .unwrap_or_else(|_| panic!("Expected ProgramOp"))
}

pub fn check_wasm(
    source: &[u8],
    input: Vec<u32>,
    secret_input: Vec<u32>,
    expected_output: u32,
    expected_wat: expect_test::Expect,
    expected_valida: expect_test::Expect,
) {
    let wat = wasmprinter::print_bytes(source).unwrap();
    expected_wat.assert_eq(&wat);

    check_valida(
        wat,
        input,
        secret_input,
        expected_output.into(),
        expected_valida,
    );
}

#[allow(unreachable_code)]
pub fn check_valida(
    source: String,
    input: Vec<u32>,
    secret_input: Vec<u32>,
    expected_output: Word<u8>,
    expected_valida: expect_test::Expect,
) {
    let wasm = wat::parse_str(source).unwrap();
    let mut ctx = Context::default();
    let source: &[u8] = &wasm;
    let target_config = ValidaTargetConfig::default();
    let prog_op = compile_to_valida_dialect(&mut ctx, source, &target_config);
    expected_valida.assert_eq(&prog_op.with_ctx(&ctx).to_string());
    let mut builder = ValidaInstrBuilder::default();
    emit_op(&ctx, prog_op.get_operation(), &mut builder);
    let program = builder.build();
    exec_valida(program, expected_output);
}

fn exec_valida(program: Vec<valida_machine::InstructionWord<i32>>, expected_output: Word<u8>) {
    use valida_cpu::MachineWithCpuChip;
    use valida_machine::Word;
    use valida_machine::{Machine, ProgramROM, PublicMemory};
    use valida_memory::MachineWithMemoryChip;

    // run valida program and check expected output
    let mut machine = BasicMachine::default();
    let rom = ProgramROM::new(program);
    let public_mem = PublicMemory::default();
    machine.cpu_mut().fp = 0x1000;
    machine.cpu_mut().save_register_state();
    machine.run(rom, public_mem);

    assert_eq!(
        *machine.mem().cells.get(&(0x1000 + 4)).unwrap(), // Return value
        expected_output
    );
}

pub fn compile_to_valida_dialect(
    ctx: &mut Context,
    source: &[u8],
    target_config: &ValidaTargetConfig,
) -> ProgramOp {
    let frontend_config = WasmFrontendConfig::default();
    frontend_config.register(ctx);
    target_config.register(ctx);
    let wasm_module_op = ozk_frontend_wasm::parse_module(ctx, source, &frontend_config).unwrap();
    run_conversion_passes(ctx, wasm_module_op, target_config)
}

pub fn check_wat(
    source: &str,
    input: Vec<u32>,
    secret_input: Vec<u32>,
    expected_output: u32,
    expected_valida: expect_test::Expect,
) {
    struct Io {
        input: Vec<u32>,
        secret_input: Vec<u32>,
        output: Vec<u32>,
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

    let ozk_stdlib_pub_input = Func::wrap(&mut store, |mut caller: Caller<'_, Io>| {
        caller.data_mut().input.pop().unwrap()
    });
    let ozk_stdlib_pub_output =
        Func::wrap(&mut store, |mut caller: Caller<'_, Io>, output: i64| {
            caller.data_mut().output.push(output as u32);
        });
    let ozk_stdlib_secret_input = Func::wrap(&mut store, |mut caller: Caller<'_, Io>| {
        caller.data_mut().secret_input.pop().unwrap()
    });
    let imports = [
        ozk_stdlib_pub_input.into(),
        ozk_stdlib_pub_output.into(),
        ozk_stdlib_secret_input.into(),
    ];
    let _ = Instance::new(&mut store, &module, &imports).unwrap();

    assert_eq!(store.data().output.first().unwrap(), &expected_output);
    check_valida(
        source.to_string(),
        input,
        secret_input,
        expected_output.into(),
        expected_valida,
    );
}
