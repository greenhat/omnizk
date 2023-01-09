use c2zk_ir::pass::IrPass;

#[derive(Default)]
pub struct LocalsToMemPass;

impl IrPass for LocalsToMemPass {
    fn run_mod_pass(&self, module: &mut c2zk_ir::ir::Module) {
        // TODO: add func prologue read current base_local_offset from the last memory address and decrement it by the declared locals count.
        // TODO: substitute all local references with memory references (base_local_offset + index).
        // base_local_offset should be put on the stack before every local access.
        // local index is used in load/store as offset from base_local_offset.
        todo!()
    }

    fn run_func_pass(&self, _func: &mut c2zk_ir::ir::Func) {
        unreachable!();
    }
}
