mod inst_buf;
use c2zk_ir::ir::Func;
pub use inst_buf::InstBuffer;

use c2zk_codegen_shared::CodegenError;
use c2zk_ir::ir::Inst;
use twenty_first::shared_math::b_field_element::BFieldElement;

use crate::TritonTargetConfig;

pub fn emit_function(
    func: &Func,
    config: &TritonTargetConfig,
    sink: &mut InstBuffer,
) -> Result<(), CodegenError> {
    for ins in func.instructions() {
        emit_inst(ins, config, sink)?;
    }
    Ok(())
}

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
        Inst::End => (), // should be eliminated at this point
        Inst::Return => sink.push(AnInstruction::Return),
        Inst::I32Const { value } => sink.push(AnInstruction::Push(felt(*value))),
        Inst::LocalGet { local_index } => (), // do nothing for now, func param access is done via stack
        Inst::I32Add => sink.push(AnInstruction::Add),
        Inst::Call { func_index } => sink.push(AnInstruction::Call(func_index.to_string())),
    }
    Ok(())
}

fn felt(v: i32) -> BFieldElement {
    // TODO: implement according to https://github.com/Neptune-Crypto/twenty-first/issues/32
    BFieldElement::new(v as u64)
}
