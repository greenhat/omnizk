pub use crate::config::*;
pub use crate::error::*;

use c2zk_ir::ir::Module;
pub fn emit(module: &Module, config: TritonTargetConfig) -> Result<Vec<u8>, TritonError> {
    todo!()
}
