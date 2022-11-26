use c2zk_codegen_shared::CodegenError;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::Inst;
use twenty_first::shared_math::b_field_element::BFieldElement;

use crate::InstBuffer;
use crate::TritonTargetConfig;

#[allow(unused_variables)]
pub fn emit_inst(
    ins: &Inst,
    config: &TritonTargetConfig,
    sink: &mut InstBuffer,
) -> Result<(), CodegenError> {
    // TODO: rename Inst to HIR and introduce MIR?
    use triton_vm::instruction::AnInstruction;
    match ins {
        Inst::Unreachable => todo!(),
        Inst::Nop => todo!(),
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
    }
    Ok(())
}

pub(crate) fn func_index_to_label(func_index: FuncIndex) -> String {
    format!("f{}", func_index.as_u32())
}

fn felt(v: i32) -> BFieldElement {
    // TODO: implement according to https://github.com/Neptune-Crypto/twenty-first/issues/32
    BFieldElement::new(v as u64)
}
