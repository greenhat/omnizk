use c2zk_ir::ir::ext::Ext;
use c2zk_ir::ir::ext::TritonExt;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::Inst;
use triton_vm::ord_n::Ord16;

use crate::codegen::pseudo_inst::*;
use crate::felt_i32;
use crate::felt_i64;
use crate::InstBuffer;
use crate::TritonError;
use crate::TritonTargetConfig;

#[allow(unused_variables)]
pub fn emit_inst(
    ins: &Inst,
    config: &TritonTargetConfig,
    sink: &mut InstBuffer,
    func_names: &[String],
) -> Result<(), TritonError> {
    use triton_vm::instruction::AnInstruction;
    match ins {
        Inst::Block { blockty } => return Err(unexpected_inst(ins)),
        Inst::Loop { block_type } => return Err(unexpected_inst(ins)),
        Inst::BrIf { relative_depth } => return Err(unexpected_inst(ins)),
        Inst::Br { relative_depth } => return Err(unexpected_inst(ins)),
        Inst::Unreachable => (),
        Inst::Nop => (),
        Inst::End => sink.push(AnInstruction::Return),
        Inst::Return => sink.push(AnInstruction::Return),
        Inst::I32Const { value } => sink.push(AnInstruction::Push(felt_i32(*value))),
        Inst::LocalGet {
            local_idx: local_index,
        } => (), // TODO: implement
        Inst::I32Add => sink.push(AnInstruction::Add),
        Inst::I32Sub => sink.append(sub_i32()),
        Inst::I64Add => sink.push(AnInstruction::Add),
        Inst::I64Mul => sink.push(AnInstruction::Mul),
        Inst::Call {
            func_idx: func_index,
        } => sink.push(AnInstruction::Call(func_index_to_label(
            *func_index,
            func_names,
        ))),
        Inst::PubInputRead => sink.push(AnInstruction::ReadIo),
        Inst::PubOutputWrite => sink.push(AnInstruction::WriteIo),
        Inst::SecretInputRead => sink.push(AnInstruction::Divine(None)),
        Inst::LocalTee { local_idx } => sink.push(AnInstruction::Nop), // TODO: implement
        Inst::I64Eqz => sink.append(vec![AnInstruction::Push(0u32.into()), AnInstruction::Eq]),
        Inst::I32Eqz => sink.append(vec![AnInstruction::Push(0u32.into()), AnInstruction::Eq]),
        Inst::I64Const { value } => sink.push(AnInstruction::Push(felt_i64(*value))),
        Inst::I64And => return Err(unexpected_inst(ins)),
        Inst::LocalSet { local_idx } => sink.push(AnInstruction::Nop), // TODO: implement
        Inst::I64GeU => sink.push(AnInstruction::Nop),                 // TODO: implement
        Inst::I64Ne => sink.push(AnInstruction::Nop),                  // TODO: implement
        Inst::Ext(Ext::Triton(eop)) => match eop {
            TritonExt::Pop => sink.push(AnInstruction::Pop),
            TritonExt::Skiz => sink.push(AnInstruction::Skiz),
            TritonExt::Swap { idx } => sink.push(AnInstruction::Swap(ord16_u8(*idx)?)),
            TritonExt::Recurse => sink.push(AnInstruction::Recurse),
            TritonExt::Lsb => sink.push(AnInstruction::Lsb),
            TritonExt::Assert => sink.push(AnInstruction::Assert),
        },
    }
    Ok(())
}

pub(crate) fn func_index_to_label(func_index: FuncIndex, func_names: &[String]) -> String {
    func_names
        .get(usize::from(func_index))
        .unwrap_or(&format!("f{}", u32::from(func_index)))
        .to_string()
}

fn ord16_u8(x: u8) -> Result<Ord16, TritonError> {
    (x as u32)
        .try_into()
        .map_err(|_| TritonError::InvalidInst(format!("invalid Ord16 index: {}", x)))
}

fn unexpected_inst(inst: &Inst) -> TritonError {
    TritonError::InvalidInst(format!("unexpected instruction: {:?}", inst))
}
