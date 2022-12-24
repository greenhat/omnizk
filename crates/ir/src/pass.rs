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

    fn run_mod_pass(&self, module: &mut Module) {
        for func in module.functions_mut().iter_mut() {
            self.run_func_pass(func);
        }
    }

    fn run_func_pass(&self, func: &mut Func);
}

pub fn run_ir_passes(module: &mut Module, passes: &[Box<dyn IrPass>]) {
    for pass in passes {
        pass.run_mod_pass(module);
    }
}
