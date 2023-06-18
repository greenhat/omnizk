use c2zk_ir::ir::Module;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::operation::Operation;

use crate::CodegenError;

pub trait Target {
    fn name(&self) -> &str;
    fn compile_module_old(&self, module: Module) -> Result<Vec<u8>, CodegenError>;
    fn compile(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<Vec<u8>, CodegenError>;
}
