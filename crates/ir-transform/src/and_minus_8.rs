use c2zk_ir::ir::Func;
use c2zk_ir::ir::Inst;
use c2zk_ir::pass::IrPass;

#[derive(Default)]
pub struct AndMinus8Pass;

impl IrPass for AndMinus8Pass {
    fn run_func_pass(&self, func: &mut Func) {
        let mut iter = func.instructions_mut().iter_mut().peekable();
        while let Some(inst) = iter.next() {
            // TODO: handle other types
            if let Inst::I32Const { value } = inst {
                if *value < 0 {
                    if let Some(Inst::I32And) = iter.peek() {
                        *value = i32::MAX - (*value).abs() - 1;
                    }
                }
            }
        }
    }
}
