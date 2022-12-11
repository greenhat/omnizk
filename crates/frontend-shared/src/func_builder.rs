use c2zk_ir::ir::Func;
use c2zk_ir::ir::Inst;

use crate::InstBuilder;

#[derive(Debug)]
pub struct FuncBuilder {
    ins: Vec<Inst>,
}

impl FuncBuilder {
    pub fn new() -> FuncBuilder {
        FuncBuilder { ins: Vec::new() }
    }

    pub fn build(self) -> Func {
        Func::new(self.ins)
    }

    pub fn ins(&mut self) -> InstBuilder {
        InstBuilder::new(self)
    }

    pub fn push_inst(&mut self, inst: Inst) {
        self.ins.push(inst);
    }

    pub fn push_insts(&mut self, insts: Vec<Inst>) {
        self.ins.extend(insts);
    }
}
