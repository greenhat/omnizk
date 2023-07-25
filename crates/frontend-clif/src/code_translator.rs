use cranelift_codegen::ir::InstructionData;
use pliron::context::Context;

use crate::func_builder::FuncBuilder;

#[allow(unused_variables)]
pub fn translate_op(ctx: &mut Context, fb: &mut FuncBuilder, inst: InstructionData) {
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
