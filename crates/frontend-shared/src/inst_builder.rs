use c2zk_ir::ir::Inst;

use crate::FuncBuilder;

pub struct InstBuilder<'a> {
    fbuilder: &'a mut FuncBuilder,
}

impl<'a> InstBuilder<'a> {
    pub fn new(fbuilder: &mut FuncBuilder) -> InstBuilder {
        InstBuilder { fbuilder }
    }

    pub fn i32const(&mut self, value: i32) {
        self.fbuilder.push_inst(Inst::I32Const { value });
    }

    pub fn ret(&mut self) {
        self.fbuilder.push_inst(Inst::Return);
    }

    pub fn end(&mut self) {
        self.fbuilder.push_inst(Inst::End);
    }

    pub fn local_get(&mut self, local_index: u32) {
        self.fbuilder.push_inst(Inst::LocalGet {
            local_idx: local_index,
        });
    }

    pub fn i32add(&mut self) {
        self.fbuilder.push_inst(Inst::I32Add);
    }

    pub fn i64add(&mut self) {
        self.fbuilder.push_inst(Inst::I32Add);
    }

    pub fn call(&mut self, func_index: u32) {
        self.fbuilder.push_inst(Inst::Call {
            func_idx: func_index.into(),
        });
    }
}
