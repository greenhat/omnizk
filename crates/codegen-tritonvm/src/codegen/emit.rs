use std::collections::HashMap;

use ozk_codegen_shared::func_index_to_label;
use ozk_ir::ir::ext::Ext;
use ozk_ir::ir::ext::TritonExt;
use ozk_ir::ir::FuncIndex;
use ozk_ir::ir::Inst;
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
    func_names: &HashMap<FuncIndex, String>,
) -> Result<(), TritonError> {
    match ins {
        Inst::Unreachable => (),
        Inst::Nop => sink.push(AnInstruction::Nop),
        Inst::End => sink.push(AnInstruction::Return),
        Inst::Return => sink.push(AnInstruction::Return),
        Inst::I32Const { value } => sink.push(AnInstruction::Push(felt_i32(*value))),
        Inst::I32Load { offset } => read_mem(sink, offset),
        Inst::I32Store { offset } => write_mem(sink, offset),
        Inst::I32Add => sink.push(AnInstruction::Add),
        Inst::I32Mul => sink.push(AnInstruction::Mul),
        Inst::I32And => sink.push(AnInstruction::And),
        Inst::I32GeU => {
            // todo!("Wasm semantics: pop i2, pop i1, push i1 >= i2");
            // todo!("Lt semantics: pop i1, pop i2, push i1 < i2");
            // todo!("extract as an IR pass with tests");
            sink.append(vec![
                // swap lhs and rhs from Wasm semantics to TritonVM Lt semantics
                AnInstruction::Swap(Ord16::ST1),
                // Duplicate the pair
                AnInstruction::Dup(Ord16::ST1),
                AnInstruction::Dup(Ord16::ST1),
                AnInstruction::Lt,
                // invert Lt to Gt
                AnInstruction::Push(0u32.into()),
                AnInstruction::Eq,
                // ----------------
                // swap Gt with second element
                AnInstruction::Swap(Ord16::ST2),
                AnInstruction::Swap(Ord16::ST1),
                AnInstruction::Eq,
                // Gt + Eq
                AnInstruction::Add,
                // Gt + Eq == 1
                AnInstruction::Push(1u32.into()),
                AnInstruction::Eq,
            ])
        }
        Inst::I32Eqz => sink.append(vec![AnInstruction::Push(0u32.into()), AnInstruction::Eq]),
        Inst::I32WrapI64 => (),
        Inst::I64Add => sink.push(AnInstruction::Add),
        Inst::I64Mul => sink.push(AnInstruction::Mul),
        Inst::I64And => sink.push(AnInstruction::And),
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
        Inst::I64Eq => sink.push(AnInstruction::Eq),
        Inst::I64Const { value } => sink.push(AnInstruction::Push(felt_i64(*value))),
        // TODO: extract to IR pass
        Inst::I64GeU => sink.append(vec![
            // swap lhs and rhs from Wasm semantics to TritonVM Lt semantics
            AnInstruction::Swap(Ord16::ST1),
            // Duplicate the pair
            AnInstruction::Dup(Ord16::ST1),
            AnInstruction::Dup(Ord16::ST1),
            AnInstruction::Lt,
            // invert Lt to Gt
            AnInstruction::Push(0u32.into()),
            AnInstruction::Eq,
            // ----------------
            // swap Gt with second element
            AnInstruction::Swap(Ord16::ST2),
            AnInstruction::Swap(Ord16::ST1),
            AnInstruction::Eq,
            // Gt + Eq
            AnInstruction::Add,
            // Gt + Eq == 1
            AnInstruction::Push(1u32.into()),
            AnInstruction::Eq,
        ]),
        // TODO: extract to IR pass
        Inst::I64Ne => sink.append(vec![
            AnInstruction::Eq,
            AnInstruction::Push(0u32.into()),
            AnInstruction::Eq,
        ]),
        Inst::I64ExtendI32U => (),
        // Extra (besides the wasm instructions)
        // -------------------------------------
        Inst::Swap { idx } => sink.push(AnInstruction::Swap(ord16_u8(*idx)?)),
        Inst::Dup { idx } => sink.push(AnInstruction::Dup(ord16_u8(*idx)?)),
        Inst::Drop => sink.push(AnInstruction::Pop),
        // Extention instructions for target arch
        Inst::Ext(Ext::Triton(eop)) => match eop {
            TritonExt::Pop => sink.push(AnInstruction::Pop),
            TritonExt::Skiz => sink.push(AnInstruction::Skiz),
            TritonExt::Recurse => sink.push(AnInstruction::Recurse),
            TritonExt::Lsb => todo!("it's pseudo op now"),
            TritonExt::Assert => sink.push(AnInstruction::Assert),
        },

        // Extention instructions for unsupported architecures
        Inst::Ext(_) => return Err(unexpected_inst(ins)),
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
        Inst::I32Sub => return Err(unexpected_inst(ins)),
    }
    Ok(())
}

fn write_mem(sink: &mut InstBuffer, offset: &u32) {
    // TODO: we're treating TritonVM memory model as KV store for now
    // meaning that the pointer is the key and the value is the value
    // ignoring the fact that Wasm pointer points to a byte
    if offset != &0 {
        // swap the value and the pointer to add the offset
        sink.push(AnInstruction::Swap(Ord16::ST1));
        sink.push(AnInstruction::Push(felt_i32(*offset as i32)));
        sink.push(AnInstruction::Add);
        // swap the value and the pointer so the value is on top of the stack
        sink.push(AnInstruction::Swap(Ord16::ST1));
    }
    sink.push(AnInstruction::WriteMem);
    // remove the pointer from the stack
    sink.push(AnInstruction::Pop);
}

fn read_mem(sink: &mut InstBuffer, offset: &u32) {
    if offset != &0 {
        sink.push(AnInstruction::Push(felt_i32(*offset as i32)));
        sink.push(AnInstruction::Add);
    }
    sink.push(AnInstruction::ReadMem);
    // swap the read value with the pointer (it's left after the read)
    sink.push(AnInstruction::Swap(Ord16::ST1));
    sink.push(AnInstruction::Pop);
}

fn ord16_u8(x: u8) -> Result<Ord16, TritonError> {
    (x as u32)
        .try_into()
        .map_err(|_| TritonError::InvalidInst(format!("invalid Ord16 index: {}", x)))
}

fn unexpected_inst(inst: &Inst) -> TritonError {
    TritonError::InvalidInst(format!("unexpected instruction: {:?}", inst))
}
