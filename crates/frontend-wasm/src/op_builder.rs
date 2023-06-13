use ozk_wasm_dialect::ops::BlockOp;
use ozk_wasm_dialect::ops::CallOp;
use ozk_wasm_dialect::ops::ConstantOp;
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
use crate::func_builder::FuncBuilderError;
use crate::types::from_block_type;

pub struct OpBuilder<'a> {
    fbuilder: &'a mut FuncBuilder,
}

#[allow(unused_variables)]
impl<'a> OpBuilder<'a> {
    fn i32_attr(ctx: &mut Context, value: i32) -> AttrObj {
        IntegerAttr::create(i32_type(ctx), value.into())
    }

    fn i64_attr(ctx: &mut Context, value: i64) -> AttrObj {
        IntegerAttr::create(i64_type(ctx), value.into())
    }

    pub fn new(fbuilder: &'a mut FuncBuilder) -> OpBuilder<'a> {
        OpBuilder { fbuilder }
    }

    pub fn i32const(&mut self, ctx: &mut Context, value: i32) -> Result<(), FuncBuilderError> {
        let val = OpBuilder::i32_attr(ctx, value);
        let op = ConstantOp::new_unlinked(ctx, val).get_operation();
        self.fbuilder.push(ctx, op)?;
        Ok(())
    }

    pub fn i64const(&mut self, ctx: &mut Context, value: i64) -> Result<(), FuncBuilderError> {
        let val = OpBuilder::i64_attr(ctx, value);
        let op = ConstantOp::new_unlinked(ctx, val).get_operation();
        self.fbuilder.push(ctx, op)?;
        Ok(())
    }

    pub fn call(&mut self, ctx: &mut Context, callee_name: String) -> Result<(), FuncBuilderError> {
        let op = CallOp::new_unlinked(ctx, callee_name).get_operation();
        self.fbuilder.push(ctx, op)?;
        Ok(())
    }

    pub fn ret(&mut self, ctx: &mut Context) -> Result<(), FuncBuilderError> {
        let op = ReturnOp::new_unlinked(ctx).get_operation();
        self.fbuilder.push(ctx, op)?;
        Ok(())
    }

    pub fn bloop(
        &mut self,
        ctx: &mut Context,
        block_type: &BlockType,
    ) -> Result<(), FuncBuilderError> {
        let ty = from_block_type(ctx, block_type);
        let op = LoopOp::new_unlinked(ctx, ty).get_operation();
        self.fbuilder.push(ctx, op)?;
        Ok(())
    }

    pub fn block(
        &mut self,
        ctx: &mut Context,
        block_type: &BlockType,
    ) -> Result<(), FuncBuilderError> {
        let ty = from_block_type(ctx, block_type);
        let op = BlockOp::new_unlinked(ctx, ty).get_operation();
        self.fbuilder.push(ctx, op)?;
        Ok(())
    }

    pub fn end(&mut self, ctx: &mut Context) -> Result<(), FuncBuilderError> {
        self.fbuilder.push_end(ctx)?;
        Ok(())
    }

    pub fn local_get(&mut self, ctx: &mut Context, local_index: u32) {
        todo!();
    }

    pub fn local_tee(&mut self, ctx: &mut Context, local_index: u32) {
        todo!();
    }

    pub fn local_set(&mut self, ctx: &mut Context, local_index: u32) {
        todo!();
    }

    pub fn i32add(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i32eqz(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i32wrapi64(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i32and(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i32geu(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i64add(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i64eqz(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i64eq(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i64and(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i64geu(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i64ne(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn i64extendi32u(&mut self, ctx: &mut Context) {
        todo!();
    }

    // pub fn call(&mut self, ctx: &mut Context, func_index: u32) {
    //     self.fbuilder.push(Inst::Call {
    //         func_idx: func_index.into(),
    //     });
    // }

    pub fn nop(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn unreachable(&mut self, ctx: &mut Context) {
        todo!();
    }

    pub fn br_if(&mut self, ctx: &mut Context, relative_depth: u32) {
        todo!();
    }

    pub fn br(&mut self, ctx: &mut Context, relative_depth: u32) {
        todo!();
    }
}
