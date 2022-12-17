use c2zk_codegen_shared::CodegenError;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::Inst;

use crate::felt;
use crate::InstBuffer;
use crate::TritonTargetConfig;

#[allow(unused_variables)]
pub fn emit_inst(
    ins: &Inst,
    config: &TritonTargetConfig,
    sink: &mut InstBuffer,
) -> Result<(), CodegenError> {
    use triton_vm::instruction::AnInstruction;
    match ins {
        Inst::Unreachable => (),
        Inst::Nop => (),
        Inst::End => sink.push(AnInstruction::Return),
        Inst::Return => sink.push(AnInstruction::Return),
        Inst::I32Const { value } => sink.push(AnInstruction::Push(felt(*value))),
        Inst::LocalGet {
            local_idx: local_index,
        } => (), // do nothing for now, func param access is done via stack
        Inst::I32Add => sink.push(AnInstruction::Add),
        Inst::I64Add => sink.push(AnInstruction::Add),
        Inst::Call {
            func_idx: func_index,
        } => sink.push(AnInstruction::Call(func_index_to_label(*func_index))),
        Inst::PubInputRead => sink.push(AnInstruction::ReadIo),
        Inst::PubOutputWrite => sink.push(AnInstruction::WriteIo),
        Inst::SecretInputRead => sink.push(AnInstruction::Divine(None)),
        Inst::Block { blockty } => todo!(),
        Inst::LocalTee { local_idx } => todo!(),
        Inst::I64Eqz => todo!(),
        Inst::I32Eqz => todo!(),
        Inst::BrIf { relative_depth } => todo!(),
        Inst::Br { relative_depth } => todo!(),
        Inst::I64Const { value } => todo!(),
        Inst::I64And => todo!(),
        Inst::LocalSet { local_idx } => todo!(),
        Inst::I64GeU => todo!(),
        Inst::Loop { block_type } => todo!(),
        Inst::I64Ne => todo!(),
    }
    Ok(())
}

pub(crate) fn func_index_to_label(func_index: FuncIndex) -> String {
    format!("f{}", u32::from(func_index))
}
