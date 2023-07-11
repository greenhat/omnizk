//! Semantic equivalence tests for the Valida VM codegen.

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
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_wat: expect_test::Expect,
    expected_valida: expect_test::Expect,
) {
    let wat = wasmprinter::print_bytes(source).unwrap();
    expected_wat.assert_eq(&wat);

    check_valida(wat, input, secret_input, expected_output, expected_valida);
}

#[allow(unreachable_code)]
pub fn check_valida(
    source: String,
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_valida: expect_test::Expect,
) {
    let wasm = wat::parse_str(source).unwrap();
    let mut ctx = Context::default();
    let source: &[u8] = &wasm;
    let target_config = ValidaTargetConfig::default();
    let prog_op = compile_to_valida_dialect(&mut ctx, source, &target_config);
    expected_valida.assert_eq(&prog_op.with_ctx(&ctx).to_string());
    let mut builder = ValidaInstrBuilder::new();
    emit_op(&ctx, prog_op.get_operation(), &mut builder).unwrap();

    let program = builder.build();

    use valida_alu_u32::{
        add::{Add32Chip, Add32Instruction, MachineWithAdd32Chip},
        mul::{MachineWithMul32Chip, Mul32Chip, Mul32Instruction},
    };
    use valida_bus::{MachineWithGeneralBus, MachineWithMemBus, MachineWithRangeBus8};
    use valida_cpu::{
        BeqInstruction, BneInstruction, Imm32Instruction, JalInstruction, JalvInstruction,
        Load32Instruction, Store32Instruction,
    };
    use valida_cpu::{CpuChip, MachineWithCpuChip};
    use valida_derive::Machine;
    use valida_machine::{
        AbstractExtensionField, AbstractField, BusArgument, Chip, Instruction, Machine, ProgramROM,
        PublicMemory,
    };
    use valida_memory::{MachineWithMemoryChip, MemoryChip};
    use valida_range::{MachineWithRangeChip, RangeCheckerChip};

    use p3_challenger::DuplexChallenger;
    use p3_maybe_rayon::*;
    use p3_merkle_tree::MerkleTreeMMCS;
    use p3_mersenne_31::Mersenne31;
    use p3_poseidon::Poseidon;
    use p3_symmetric::compression::TruncatedPermutation;
    use p3_symmetric::mds::NaiveMDSMatrix;
    use p3_symmetric::sponge::PaddingFreeSponge;
    use p3_tensor_pcs::TensorPCS;
    use rand::thread_rng;
    use valida_machine::config::StarkConfigImpl;
    use valida_machine::Operands;
    use valida_machine::{InstructionWord, Word};

    // run valida program and check expected output
    let mut machine = BasicMachine::default();
    let rom = ProgramROM::new(program);
    let public_mem = PublicMemory::default();
    machine.cpu_mut().fp = 0x1000;
    machine.cpu_mut().save_register_state(); // TODO: Initial register state should be saved
                                             // automatically by the machine, not manually here
    machine.run(rom, public_mem);

    type Val = Mersenne31;
    type Challenge = Val; // TODO
    type PackedChallenge = Challenge; // TODO

    let mds = NaiveMDSMatrix::<Val, 8>::new([[Val::ZERO; 8]; 8]); // TODO: Use a real MDS matrix
    type Perm = Poseidon<Val, NaiveMDSMatrix<Val, 8>, 8, 7>;
    let perm = Perm::new_from_rng(5, 5, mds, &mut thread_rng()); // TODO: Use deterministic RNG
    let h4 = PaddingFreeSponge::<Val, Perm, { 4 + 4 }>::new(perm.clone());
    let c = TruncatedPermutation::<Val, Perm, 2, 4, { 2 * 4 }>::new(perm.clone());
    let mmcs = MerkleTreeMMCS::new(h4, c);
    let codes = p3_brakedown::fast_registry();
    let pcs = TensorPCS::new(codes, mmcs);
    let challenger = DuplexChallenger::new(perm);
    let config = StarkConfigImpl::<Val, Challenge, PackedChallenge, _, _>::new(pcs, challenger);
    machine.prove(&config);

    assert_eq!(machine.cpu().clock, 191);
    assert_eq!(machine.cpu().operations.len(), 191);
    assert_eq!(machine.mem().operations.values().flatten().count(), 401);
    assert_eq!(machine.add_u32().operations.len(), 50);

    assert_eq!(
        *machine.mem().cells.get(&(0x1000 + 4)).unwrap(), // Return value
        Word([0, 1, 37, 17,])                             // 25th fibonacci number (75025)
    );
}

pub fn compile(ctx: &mut Context, source: &[u8]) -> String {
    let target_config = ValidaTargetConfig::default();
    let prog_op = compile_to_valida_dialect(ctx, source, &target_config);
    let mut builder = ValidaInstrBuilder::new();
    emit_op(ctx, prog_op.get_operation(), &mut builder).unwrap();
    builder.pretty_print()
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
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_valida: expect_test::Expect,
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
    check_valida(
        source.to_string(),
        input,
        secret_input,
        expected_output,
        expected_valida,
    );
}
