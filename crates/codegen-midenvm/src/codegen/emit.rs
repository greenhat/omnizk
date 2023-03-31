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
    todo!();
}
