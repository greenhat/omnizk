use c2zk_ir::ir::Inst;

use crate::InstBuffer;
use crate::MidenAssemblyBuilder;
use crate::MidenError;
use crate::MidenTargetConfig;

#[allow(unused_variables)]
pub fn emit_inst(
    ins: &Inst,
    config: &MidenTargetConfig,
    sink: &mut InstBuffer,
    func_names: &[String],
) -> Result<(), MidenError> {
    let b = MidenAssemblyBuilder::new();
    #[allow(clippy::wildcard_enum_match_arm)]
    match ins {
        Inst::End => sink.push(b.end()),
        Inst::Return => (), // TODO: this is vaid only if next inst is End
        Inst::I32Const { value } => sink.push(b.push(*value as i64)),
        Inst::I32Add => sink.push(b.add()),
        _ => return Err(MidenError::UnsupportedInstruction(ins.clone())),
    };
    Ok(())
}
