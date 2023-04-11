use c2zk_ir::ir::ext::Ext;
use c2zk_ir::ir::ext::MidenExt;
use c2zk_ir::ir::Inst;
use thiserror::Error;

use crate::InstBuffer;
use crate::MidenAssemblyBuilder;
use crate::MidenTargetConfig;

#[derive(Debug, Error)]
pub enum EmitError {
    #[error("Unsupported instruction: {0:?}")]
    UnsupportedInstruction(Inst),
}

#[allow(unused_variables)]
pub fn emit_inst(
    ins: &Inst,
    config: &MidenTargetConfig,
    sink: &mut InstBuffer,
    func_names: &[String],
) -> Result<(), EmitError> {
    let b = MidenAssemblyBuilder::new();
    #[allow(clippy::wildcard_enum_match_arm)]
    match ins {
        Inst::End => sink.push(b.end()),
        Inst::Return => (), // TODO: this is vaid only if next inst is End
        Inst::Dup { idx } => sink.push(b.dup(*idx)),
        Inst::Swap { idx } => sink.push(b.swap(*idx)),
        Inst::I32Const { value } => sink.push(b.push(*value as i64)),
        Inst::I32Add => sink.push(b.add()),
        Inst::Ext(Ext::Miden(miden_inst)) => match miden_inst {
            MidenExt::SDepth => sink.push(b.sdepth()),
            MidenExt::While => sink.push(b.while_true()),
            MidenExt::End => sink.push(b.end()),
        },
        _ => return Err(EmitError::UnsupportedInstruction(ins.clone())),
    };
    Ok(())
}
