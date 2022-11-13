use c2zk_codegen_shared::CodegenError;
use c2zk_ir::ir::Inst;

use crate::TritonTargetConfig;

#[allow(unused_variables)]
pub fn codegen(ins: &Inst, config: &TritonTargetConfig) -> Result<Vec<u8>, CodegenError> {
    match ins {
        Inst::Unreachable => todo!(),
        Inst::Nop => todo!(),
        Inst::End => todo!(),
        Inst::Return => todo!(),
        Inst::I32Const { value } => todo!(),
    }
    Ok(vec![])
}
