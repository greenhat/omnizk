use c2zk_ir::ir::Func;

use crate::CodegenError;

pub trait Target {
    fn name(&self) -> &str;
    fn compile_function(&self, func: &Func) -> Result<Vec<u8>, CodegenError>;
}
