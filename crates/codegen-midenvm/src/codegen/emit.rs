use c2zk_ir::ir::Inst;

use crate::InstBuffer;
use crate::MidenError;
use crate::MidenTargetConfig;

#[allow(unused_variables)]
pub fn emit_inst(
    ins: &Inst,
    config: &MidenTargetConfig,
    sink: &mut InstBuffer,
    func_names: &[String],
) -> Result<(), MidenError> {
    match ins {
        Inst::Unreachable => todo!(),
        Inst::Nop => todo!(),
        Inst::Call { func_idx } => todo!(),
        Inst::End => todo!(),
        Inst::Return => todo!(),
        Inst::Loop { block_type } => todo!(),
        Inst::Block { blockty } => todo!(),
        Inst::BrIf { relative_depth } => todo!(),
        Inst::Br { relative_depth } => todo!(),
        Inst::I32Const { value } => todo!(),
        Inst::I64Const { value } => todo!(),
        Inst::GlobalGet { global_idx } => todo!(),
        Inst::GlobalSet { global_idx } => todo!(),
        Inst::LocalGet { local_idx } => todo!(),
        Inst::LocalTee { local_idx } => todo!(),
        Inst::LocalSet { local_idx } => todo!(),
        Inst::I32Load { offset } => todo!(),
        Inst::I32Store { offset } => todo!(),
        Inst::I32Add => todo!(),
        Inst::I32Sub => todo!(),
        Inst::I32Mul => todo!(),
        Inst::I32Eqz => todo!(),
        Inst::I32WrapI64 => todo!(),
        Inst::I32And => todo!(),
        Inst::I32GeU => todo!(),
        Inst::I64Add => todo!(),
        Inst::I64Mul => todo!(),
        Inst::I64Eqz => todo!(),
        Inst::I64And => todo!(),
        Inst::I64GeU => todo!(),
        Inst::I64Ne => todo!(),
        Inst::I64Eq => todo!(),
        Inst::I64ExtendI32U => todo!(),
        Inst::PubInputRead => todo!(),
        Inst::PubOutputWrite => todo!(),
        Inst::SecretInputRead => todo!(),
        Inst::Swap { idx } => todo!(),
        Inst::Dup { idx } => todo!(),
        Inst::Ext(_) => todo!(),
    }
}
