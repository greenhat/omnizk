use c2zk_ir::ir::Inst;

use crate::InstBuilder;

pub struct FuncBuilder {
    ins: Vec<Inst>,
}

impl FuncBuilder {
    pub fn new() -> FuncBuilder {
        FuncBuilder { ins: Vec::new() }
    }

    pub fn ins(&mut self) -> InstBuilder {
        InstBuilder::new(self)
    }

    pub fn add_inst(&mut self, inst: Inst) {
        self.ins.push(inst);
    }
}
