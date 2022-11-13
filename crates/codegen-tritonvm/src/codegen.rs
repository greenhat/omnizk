use c2zk_codegen_shared::CodegenError;
use c2zk_ir::ir::Inst;
use triton_vm::instruction::Instruction;
use twenty_first::shared_math::b_field_element::BFieldElement;

use crate::TritonTargetConfig;

#[allow(unused_variables)]
pub fn emit(ins: &Inst, config: &TritonTargetConfig) -> Result<Vec<u8>, CodegenError> {
    let mut code = Vec::new();
    match ins {
        Inst::Unreachable => todo!(),
        Inst::Nop => todo!(),
        Inst::End => todo!(),
        Inst::Return => todo!(),
        Inst::I32Const { value } => code.push(Instruction::Push(felt(*value))),
    }
    Ok(serialize(code))
}

fn felt(v: i32) -> BFieldElement {
    // TODO: implement according to https://github.com/Neptune-Crypto/twenty-first/issues/32
    BFieldElement::new(v as u64)
}

fn serialize(_code: Vec<Instruction>) -> Vec<u8> {
    todo!()
}
