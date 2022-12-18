use c2zk_ir::ir::Func;
use c2zk_ir::ir::Module;
use c2zk_ir::pass::IrPass;

pub struct ConvertBlocksPass;

impl ConvertBlocksPass {
    pub fn new() -> Self {
        ConvertBlocksPass {}
    }
}

impl IrPass for ConvertBlocksPass {
    fn run_pass(&self, func: &mut Func, module: &mut Module) {
        todo!()
    }
}
