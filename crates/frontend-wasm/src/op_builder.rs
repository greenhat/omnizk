use ozk_wasm_dialect::ops::BlockOp;
use ozk_wasm_dialect::ops::CallOp;
use ozk_wasm_dialect::ops::ConstOp;
use ozk_wasm_dialect::ops::LoopOp;
use ozk_wasm_dialect::ops::ReturnOp;
use ozk_wasm_dialect::types::i32_type;
use ozk_wasm_dialect::types::i64_type;
use pliron::attribute::AttrObj;
use pliron::context::Context;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::op::Op;
use wasmparser::BlockType;

use crate::func_builder::FuncBuilder;
use crate::types::from_block_type;

pub struct OpBuilder<'a> {
    ctx: &'a mut Context,
    fbuilder: &'a mut FuncBuilder<'a>,
}

impl<'a> OpBuilder<'a> {
    fn i32_attr(&self, value: i32) -> AttrObj {
        IntegerAttr::create(i32_type(self.ctx), value.into())
    }

    fn i64_attr(&self, value: i64) -> AttrObj {
        IntegerAttr::create(i64_type(self.ctx), value.into())
    }

    pub fn new(ctx: &mut Context, fbuilder: &mut FuncBuilder) -> OpBuilder<'a> {
        OpBuilder { fbuilder, ctx }
    }

    pub fn i32const(&mut self, value: i32) {
        self.fbuilder
            .push(ConstOp::new_unlinked(self.ctx, self.i32_attr(value)).get_operation());
    }

    pub fn i64const(&mut self, value: i64) {
        self.fbuilder
            .push(ConstOp::new_unlinked(self.ctx, self.i64_attr(value)).get_operation());
    }

    pub fn call(&mut self, callee_name: String) {
        self.fbuilder
            .push(CallOp::new_unlinked(self.ctx, callee_name).get_operation());
    }

    pub fn ret(&mut self) {
        self.fbuilder
            .push(ReturnOp::new_unlinked(self.ctx).get_operation());
    }

    pub fn bloop(&mut self, block_type: &BlockType) {
        self.fbuilder.push(
            LoopOp::new_unlinked(self.ctx, from_block_type(self.ctx, block_type)).get_operation(),
        );
    }

    pub fn block(&mut self, block_type: &BlockType) {
        self.fbuilder.push(
            BlockOp::new_unlinked(self.ctx, from_block_type(self.ctx, block_type)).get_operation(),
        );
    }

    pub fn end(&mut self) {
        self.fbuilder.push_end();
    }

    pub fn local_get(&mut self, local_index: u32) {
        self.fbuilder.push(Inst::LocalGet {
            local_idx: local_index,
        });
    }

    pub fn local_tee(&mut self, local_index: u32) {
        self.fbuilder.push(Inst::LocalTee {
            local_idx: local_index,
        });
    }

    pub fn local_set(&mut self, local_index: u32) {
        self.fbuilder.push(Inst::LocalSet {
            local_idx: local_index,
        });
    }

    pub fn i32add(&mut self) {
        self.fbuilder.push(Inst::I32Add);
    }

    pub fn i32eqz(&mut self) {
        self.fbuilder.push(Inst::I32Eqz);
    }

    pub fn i32wrapi64(&mut self) {
        self.fbuilder.push(Inst::I32WrapI64);
    }

    pub fn i32and(&mut self) {
        self.fbuilder.push(Inst::I32And);
    }

    pub fn i32geu(&mut self) {
        self.fbuilder.push(Inst::I32GeU);
    }

    pub fn i64add(&mut self) {
        self.fbuilder.push(Inst::I32Add);
    }

    pub fn i64eqz(&mut self) {
        self.fbuilder.push(Inst::I64Eqz);
    }

    pub fn i64eq(&mut self) {
        self.fbuilder.push(Inst::I64Eq);
    }

    pub fn i64and(&mut self) {
        self.fbuilder.push(Inst::I64And);
    }

    pub fn i64geu(&mut self) {
        self.fbuilder.push(Inst::I64GeU);
    }

    pub fn i64ne(&mut self) {
        self.fbuilder.push(Inst::I64Ne);
    }

    pub fn i64extendi32u(&mut self) {
        self.fbuilder.push(Inst::I64ExtendI32U);
    }

    // pub fn call(&mut self, func_index: u32) {
    //     self.fbuilder.push(Inst::Call {
    //         func_idx: func_index.into(),
    //     });
    // }

    pub fn nop(&mut self) {
        self.fbuilder.push(Inst::Nop);
    }

    pub fn unreachable(&mut self) {
        self.fbuilder.push(Inst::Unreachable);
    }

    pub fn br_if(&mut self, relative_depth: u32) {
        self.fbuilder.push(Inst::BrIf { relative_depth });
    }

    pub fn br(&mut self, relative_depth: u32) {
        self.fbuilder.push(Inst::Br { relative_depth });
    }
}
