mod inst_buf;
pub use inst_buf::InstBuffer;

use c2zk_codegen_shared::CodegenError;
use c2zk_ir::ir::Inst;
use triton_vm::instruction::Instruction;
use twenty_first::shared_math::b_field_element::BFieldElement;

use crate::TritonTargetConfig;

#[allow(unused_variables)]
pub fn emit(
    ins: &Inst,
    config: &TritonTargetConfig,
    sink: &mut InstBuffer,
) -> Result<(), CodegenError> {
    match ins {
        Inst::Unreachable => todo!(),
        Inst::Nop => todo!(),
        Inst::End => (), // should be eliminated at this point
        Inst::Return => sink.push(Instruction::Return),
        Inst::I32Const { value } => sink.push(Instruction::Push(felt(*value))),
    }
    Ok(())
}

fn felt(v: i32) -> BFieldElement {
    // TODO: implement according to https://github.com/Neptune-Crypto/twenty-first/issues/32
    BFieldElement::new(v as u64)
}
