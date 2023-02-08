use c2zk_ir::ir::Module;

use crate::CodegenError;

pub trait Target {
    fn name(&self) -> &str;
    fn compile_module(&self, module: Module) -> Result<Vec<u8>, CodegenError>;
}
