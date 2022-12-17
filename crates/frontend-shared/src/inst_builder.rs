use c2zk_ir::ir::BlockType;
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

    pub fn i64const(&mut self, value: i64) {
        self.fbuilder.push_inst(Inst::I64Const { value });
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

    pub fn local_tee(&mut self, local_index: u32) {
        self.fbuilder.push_inst(Inst::LocalTee {
            local_idx: local_index,
        });
    }

    pub fn local_set(&mut self, local_index: u32) {
        self.fbuilder.push_inst(Inst::LocalSet {
            local_idx: local_index,
        });
    }

    pub fn i32add(&mut self) {
        self.fbuilder.push_inst(Inst::I32Add);
    }

    pub fn i32eqz(&mut self) {
        self.fbuilder.push_inst(Inst::I32Eqz);
    }

    pub fn i64add(&mut self) {
        self.fbuilder.push_inst(Inst::I32Add);
    }

    pub fn i64eqz(&mut self) {
        self.fbuilder.push_inst(Inst::I64Eqz);
    }

    pub fn i64and(&mut self) {
        self.fbuilder.push_inst(Inst::I64And);
    }

    pub fn i64geu(&mut self) {
        self.fbuilder.push_inst(Inst::I64GeU);
    }

    pub fn i64ne(&mut self) {
        self.fbuilder.push_inst(Inst::I64Ne);
    }

    pub fn call(&mut self, func_index: u32) {
        self.fbuilder.push_inst(Inst::Call {
            func_idx: func_index.into(),
        });
    }

    pub fn nop(&mut self) {
        self.fbuilder.push_inst(Inst::Nop);
    }

    pub fn unreachable(&mut self) {
        self.fbuilder.push_inst(Inst::Unreachable);
    }

    pub fn bloop(&mut self, block_type: BlockType) {
        self.fbuilder.push_inst(Inst::Loop { block_type });
    }

    pub fn block(&mut self, blockty: BlockType) {
        self.fbuilder.push_inst(Inst::Block { blockty });
    }

    pub fn br_if(&mut self, relative_depth: u32) {
        self.fbuilder.push_inst(Inst::BrIf { relative_depth });
    }

    pub fn br(&mut self, relative_depth: u32) {
        self.fbuilder.push_inst(Inst::Br { relative_depth });
    }
}
