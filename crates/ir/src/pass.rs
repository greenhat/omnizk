//! Passes are transformations that can be applied to the IR.

use crate::ir::Func;
use crate::ir::Module;

pub trait IrPass {
    fn name(&self) -> &str {
        let name = std::any::type_name::<Self>();
        if let Some((_, tail)) = name.rsplit_once(':') {
            tail
        } else {
            name
        }
    }

    // TODO: some ModuleCtx(add new funcs, query info, etc.) instead of Module
    fn run_pass(&self, func: &mut Func, module: &mut Module);
}
