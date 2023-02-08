use c2zk_ir::ir::ext::Ext;
use c2zk_ir::ir::ext::TritonExt;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::Inst;
use triton_opcodes::instruction::AnInstruction;
use triton_opcodes::ord_n::Ord16;

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
    match ins {
        Inst::Unreachable => (),
        Inst::Nop => (),
        Inst::End => sink.push(AnInstruction::Return),
        Inst::Return => sink.push(AnInstruction::Return),
        Inst::I32Const { value } => sink.push(AnInstruction::Push(felt_i32(*value))),
        Inst::I32Load { offset } => read_mem(sink, offset),
        Inst::I32Store { offset } => write_mem(sink, offset),
        Inst::I32Add => sink.push(AnInstruction::Add),
        Inst::I32Mul => sink.push(AnInstruction::Mul),
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
        Inst::I64Eqz => sink.append(vec![AnInstruction::Push(0u32.into()), AnInstruction::Eq]),
        Inst::I32Eqz => sink.append(vec![AnInstruction::Push(0u32.into()), AnInstruction::Eq]),
        Inst::I64Const { value } => sink.push(AnInstruction::Push(felt_i64(*value))),
        Inst::I64GeU => sink.push(AnInstruction::Nop), // TODO: implement
        Inst::I64Ne => sink.push(AnInstruction::Nop),  // TODO: implement
        // Extra (besides the wasm instructions)
        // -------------------------------------
        Inst::Swap { idx } => sink.push(AnInstruction::Swap(ord16_u8(*idx)?)),
        Inst::Dup { idx } => sink.push(AnInstruction::Dup(ord16_u8(*idx)?)),
        // Extention instructions for target arch
        Inst::Ext(Ext::Triton(eop)) => match eop {
            TritonExt::Pop => sink.push(AnInstruction::Pop),
            TritonExt::Skiz => sink.push(AnInstruction::Skiz),
            TritonExt::Recurse => sink.push(AnInstruction::Recurse),
            TritonExt::Lsb => todo!("it's pseudo op now"),
            TritonExt::Assert => sink.push(AnInstruction::Assert),
        },
        // Should not be emitted (eliminated in the IR transformation passes)
        Inst::Block { blockty } => return Err(unexpected_inst(ins)),
        Inst::Loop { block_type } => return Err(unexpected_inst(ins)),
        Inst::BrIf { relative_depth } => return Err(unexpected_inst(ins)),
        Inst::Br { relative_depth } => return Err(unexpected_inst(ins)),
        Inst::LocalGet { local_idx } => return Err(unexpected_inst(ins)),
        Inst::LocalSet { local_idx } => return Err(unexpected_inst(ins)),
        Inst::LocalTee { local_idx } => return Err(unexpected_inst(ins)),
        Inst::GlobalGet { global_idx } => return Err(unexpected_inst(ins)),
        Inst::GlobalSet { global_idx } => return Err(unexpected_inst(ins)),
        Inst::I64And => return Err(unexpected_inst(ins)),
        Inst::I32Sub => return Err(unexpected_inst(ins)),
    }
    Ok(())
}

fn write_mem(sink: &mut InstBuffer, offset: &u32) {
    if offset != &0 {
        sink.push(AnInstruction::Push(felt_i32(*offset as i32)));
        sink.push(AnInstruction::Add);
    }
    sink.push(AnInstruction::WriteMem);
    // remove the top two elements from the stack (the value and the pointer)
    sink.push(AnInstruction::Pop);
    sink.push(AnInstruction::Pop);
}

fn read_mem(sink: &mut InstBuffer, offset: &u32) {
    if offset != &0 {
        sink.push(AnInstruction::Push(felt_i32(*offset as i32)));
        sink.push(AnInstruction::Add);
    }
    // push 0 on top of the stack since read_mem overrites the top of the stack with the read value
    sink.push(AnInstruction::Push(felt_i32(0)));
    sink.push(AnInstruction::ReadMem);
    // swap the read value with the pointer (it's left after the read)
    sink.push(AnInstruction::Swap(Ord16::ST1));
    sink.push(AnInstruction::Pop);
}

// fn global_get(sink: &mut InstBuffer, global_idx: &GlobalIndex) {
//     // TODO: extract this into a function and call it instead of "inlining"
//     sink.append(vec![
//         AnInstruction::Push(felt_i32(GLOBAL_MEMORY_BASE as i32)),
//         AnInstruction::Push(felt_i32(-(u32::from(*global_idx) as i32))),
//         AnInstruction::Add,
//         AnInstruction::ReadMem,
//     ])
// }

// fn global_set(sink: &mut InstBuffer, global_idx: &GlobalIndex) {
//     // TODO: extract this into a function and call it instead of "inlining"
//     sink.append(vec![
//         AnInstruction::Push(felt_i32(GLOBAL_MEMORY_BASE as i32)),
//         AnInstruction::Push(felt_i32(-(u32::from(*global_idx) as i32))),
//         AnInstruction::Add,
//         AnInstruction::WriteMem,
//     ])
// }

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
