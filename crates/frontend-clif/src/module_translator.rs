use cranelift_codegen::entity::EntityRef;
use cranelift_codegen::ir::InstructionData;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::isa::TargetFrontendConfig;
use cranelift_wasm::DummyEnvironment;
use cranelift_wasm::WasmError;
use ozk_wasm_dialect as wasm;
use pliron::context::Context;
use target_lexicon::PointerWidth;
use thiserror::Error;

use crate::func_builder::FuncBuilder;

pub fn parse_module(ctx: &mut Context, wasm: &[u8]) -> Result<wasm::ops::ModuleOp, ModuleError> {
    let cranelift_config = TargetFrontendConfig {
        default_call_conv: CallConv::SystemV,
        pointer_width: PointerWidth::U32,
    };
    let mut dummy_environ = DummyEnvironment::new(cranelift_config, true);
    cranelift_wasm::translate_module(wasm, &mut dummy_environ)?;
    let num_func_imports = dummy_environ.get_num_func_imports();
    for (def_index, func) in dummy_environ.info.function_bodies.iter() {
        let func_index = num_func_imports + def_index.index();
        if let Some(start_func) = dummy_environ.info.start_func {
            if func_index == start_func.index() {
                todo!("start func");
            }
        }
        build_func(ctx, func)?;
    }
    todo!()
}

fn build_func(
    ctx: &mut Context,
    func: &cranelift_codegen::ir::Function,
) -> Result<FuncBuilder, ModuleError> {
    let mut fb = FuncBuilder::new(ctx, func.name.to_string().into());
    // TODO: set signature
    for clif_block in func.layout.blocks() {
        if func.layout.first_inst(clif_block).is_none() {
            todo!("Error: Empty block");
        }
        // TODO: get label
        let label = format!("block_{}", clif_block.as_u32());
        let block = fb.create_block(ctx, Some(label));
        fb.switch_to_block(block);
        for inst in func.layout.block_insts(clif_block) {
            let inst_data = func.dfg.insts[inst];
            // let dfg = &func.dfg;
            // let results = dfg.inst_results(inst);
            build_op(ctx, &mut fb, inst_data);
        }
    }
    Ok(fb)
}

fn build_op(ctx: &mut Context, fb: &mut FuncBuilder, inst: InstructionData) {
    match inst {
        InstructionData::AtomicCas {
            opcode,
            args,
            flags,
        } => todo!(),
        InstructionData::AtomicRmw {
            opcode,
            args,
            flags,
            op,
        } => todo!(),
        InstructionData::Binary { opcode, args } => todo!(),
        InstructionData::BinaryImm64 { opcode, arg, imm } => todo!(),
        InstructionData::BinaryImm8 { opcode, arg, imm } => todo!(),
        InstructionData::BranchTable { opcode, arg, table } => todo!(),
        InstructionData::Brif {
            opcode,
            arg,
            blocks,
        } => todo!(),
        InstructionData::Call {
            opcode,
            args,
            func_ref,
        } => todo!(),
        InstructionData::CallIndirect {
            opcode,
            args,
            sig_ref,
        } => todo!(),
        InstructionData::CondTrap { opcode, arg, code } => todo!(),
        InstructionData::DynamicStackLoad {
            opcode,
            dynamic_stack_slot,
        } => todo!(),
        InstructionData::DynamicStackStore {
            opcode,
            arg,
            dynamic_stack_slot,
        } => todo!(),
        InstructionData::FloatCompare { opcode, args, cond } => todo!(),
        InstructionData::FuncAddr { opcode, func_ref } => todo!(),
        InstructionData::IntAddTrap { opcode, args, code } => todo!(),
        InstructionData::IntCompare { opcode, args, cond } => todo!(),
        InstructionData::IntCompareImm {
            opcode,
            arg,
            cond,
            imm,
        } => todo!(),
        InstructionData::Jump {
            opcode,
            destination,
        } => todo!(),
        InstructionData::Load {
            opcode,
            arg,
            flags,
            offset,
        } => todo!(),
        InstructionData::LoadNoOffset { opcode, arg, flags } => todo!(),
        InstructionData::MultiAry { opcode, args } => todo!(),
        InstructionData::NullAry { opcode } => todo!(),
        InstructionData::Shuffle { opcode, args, imm } => todo!(),
        InstructionData::StackLoad {
            opcode,
            stack_slot,
            offset,
        } => todo!(),
        InstructionData::StackStore {
            opcode,
            arg,
            stack_slot,
            offset,
        } => todo!(),
        InstructionData::Store {
            opcode,
            args,
            flags,
            offset,
        } => todo!(),
        InstructionData::StoreNoOffset {
            opcode,
            args,
            flags,
        } => todo!(),
        InstructionData::TableAddr {
            opcode,
            arg,
            table,
            offset,
        } => todo!(),
        InstructionData::Ternary { opcode, args } => todo!(),
        InstructionData::TernaryImm8 { opcode, args, imm } => todo!(),
        InstructionData::Trap { opcode, code } => todo!(),
        InstructionData::Unary { opcode, arg } => todo!(),
        InstructionData::UnaryConst {
            opcode,
            constant_handle,
        } => todo!(),
        InstructionData::UnaryGlobalValue {
            opcode,
            global_value,
        } => todo!(),
        InstructionData::UnaryIeee32 { opcode, imm } => todo!(),
        InstructionData::UnaryIeee64 { opcode, imm } => todo!(),
        InstructionData::UnaryImm { opcode, imm } => {
            assert!(i32::try_from(imm.bits()).is_ok(), "only i32 supported");
            fb.op().i32const(ctx, i64::from(imm) as i32);
        }
    }
}

#[derive(Debug, Error)]
pub enum ModuleError {
    #[error("Wasm(CLIF) parsing error: {0}")]
    Wasm(#[from] WasmError),
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use expect_test::expect;
    use pliron::with_context::AttachContext;

    use crate::config::ClifFrontendConfig;

    use super::*;

    pub fn check_ir(wat: &str, expected_tree: expect_test::Expect) {
        let source = wat::parse_str(wat).unwrap();
        let mut ctx = Context::default();
        let frontend_config = ClifFrontendConfig::default();
        frontend_config.register(&mut ctx);
        let wasm_module_op = parse_module(&mut ctx, &source).unwrap();
        expected_tree.assert_eq(wasm_module_op.with_ctx(&ctx).to_string().as_str());
    }

    #[test]
    fn smoke() {
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
            }"#]],
        );
    }
}
