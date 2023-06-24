use thiserror::Error;

use crate::ValidaTargetConfig;

#[derive(Debug, Error)]
pub enum EmitError {}

#[allow(unused_variables)]
pub fn emit_inst(
    // inst_iter: &mut impl Iterator<Item = Inst>,
    config: &ValidaTargetConfig,
    // sink: &mut InstBuffer,
    // func_names: &HashMap<FuncIndex, String>,
) -> Result<(), EmitError> {
    todo!()
}
