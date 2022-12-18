use c2zk_ir::ir::Module;
use c2zk_ir::pass::IrPass;

use crate::CodegenError;

pub trait Target {
    fn name(&self) -> &str;
    fn ir_passes(&self) -> Vec<Box<dyn IrPass>>;
    fn compile_module(&self, module: Module) -> Result<Vec<u8>, CodegenError>;
}
