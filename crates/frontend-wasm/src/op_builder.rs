use ozk_ozk_dialect::attributes::i32_attr;
use ozk_ozk_dialect::attributes::i64_attr;
use ozk_ozk_dialect::types::i32_type;
use ozk_ozk_dialect::types::i64_type;
use ozk_wasm_dialect::ops::AddOp;
use ozk_wasm_dialect::ops::BlockOp;
use ozk_wasm_dialect::ops::BrIfOp;
use ozk_wasm_dialect::ops::BrOp;
use ozk_wasm_dialect::ops::CallOp;
use ozk_wasm_dialect::ops::ConstantOp;
use ozk_wasm_dialect::ops::GlobalGetOp;
use ozk_wasm_dialect::ops::GlobalSetOp;
use ozk_wasm_dialect::ops::I32EqzOp;
use ozk_wasm_dialect::ops::LocalGetOp;
use ozk_wasm_dialect::ops::LocalSetOp;
use ozk_wasm_dialect::ops::LocalTeeOp;
use ozk_wasm_dialect::ops::LoopOp;
use ozk_wasm_dialect::ops::ReturnOp;
use ozk_wasm_dialect::types::from_block_type;
use pliron::context::Context;
use pliron::op::Op;
use wasmparser::BlockType;

use crate::func_builder::FuncBuilder;
use crate::func_builder::FuncBuilderError;

// TODO: remove FuncBuilder dep and move to wasm dialect crate?
pub struct OpBuilder<'a> {
    fbuilder: &'a mut FuncBuilder,
}

#[allow(unused_variables)]
impl<'a> OpBuilder<'a> {
    pub fn new(fbuilder: &'a mut FuncBuilder) -> OpBuilder<'a> {
        OpBuilder { fbuilder }
    }

    pub fn i32const(&mut self, ctx: &mut Context, value: i32) -> Result<(), FuncBuilderError> {
        let val = i32_attr(ctx, value);
        let op = ConstantOp::new_unlinked(ctx, val).get_operation();
        self.fbuilder.push(ctx, op)?;
        Ok(())
    }

    pub fn i64const(&mut self, ctx: &mut Context, value: i64) -> Result<(), FuncBuilderError> {
        let val = i64_attr(ctx, value);
        let op = ConstantOp::new_unlinked(ctx, val).get_operation();
        self.fbuilder.push(ctx, op)?;
        Ok(())
    }

    pub fn call(&mut self, ctx: &mut Context, func_index: u32) -> Result<(), FuncBuilderError> {
        let op = CallOp::new_unlinked(ctx, func_index.into()).get_operation();
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
        self.fbuilder.push_end(ctx)
    }

    pub fn global_set(
        &mut self,
        ctx: &mut Context,
        global_index: u32,
    ) -> Result<(), FuncBuilderError> {
        let op = GlobalSetOp::new_unlinked(ctx, global_index.into());
        self.fbuilder.push(ctx, op.get_operation())
    }

    pub fn global_get(
        &mut self,
        ctx: &mut Context,
        global_index: u32,
    ) -> Result<(), FuncBuilderError> {
        let op = GlobalGetOp::new_unlinked(ctx, global_index);
        self.fbuilder.push(ctx, op.get_operation())
    }

    pub fn local_get(
        &mut self,
        ctx: &mut Context,
        local_index: u32,
    ) -> Result<(), FuncBuilderError> {
        let op = LocalGetOp::new_unlinked(ctx, local_index);
        self.fbuilder.push(ctx, op.get_operation())
    }

    pub fn local_tee(
        &mut self,
        ctx: &mut Context,
        local_index: u32,
    ) -> Result<(), FuncBuilderError> {
        let op = LocalTeeOp::new_unlinked(ctx, local_index);
        self.fbuilder.push(ctx, op.get_operation())
    }

    pub fn local_set(
        &mut self,
        ctx: &mut Context,
        local_index: u32,
    ) -> Result<(), FuncBuilderError> {
        let op = LocalSetOp::new_unlinked(ctx, local_index);
        self.fbuilder.push(ctx, op.get_operation())
    }

    pub fn i32add(&mut self, ctx: &mut Context) -> Result<(), FuncBuilderError> {
        let ty = i32_type(ctx);
        let op = AddOp::new_unlinked(ctx, ty).get_operation();
        self.fbuilder.push(ctx, op)
    }

    pub fn i32eqz(&mut self, ctx: &mut Context) -> Result<(), FuncBuilderError> {
        let op = I32EqzOp::new_unlinked(ctx).get_operation();
        self.fbuilder.push(ctx, op)
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

    pub fn i64add(&mut self, ctx: &mut Context) -> Result<(), FuncBuilderError> {
        let ty = i64_type(ctx);
        let op = AddOp::new_unlinked(ctx, ty).get_operation();
        self.fbuilder.push(ctx, op)
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

    pub fn br_if(
        &mut self,
        ctx: &mut Context,
        relative_depth: u32,
    ) -> Result<(), FuncBuilderError> {
        let op = BrIfOp::new_unlinked(ctx, relative_depth.into());
        self.fbuilder.push(ctx, op.get_operation())
    }

    pub fn br(&mut self, ctx: &mut Context, relative_depth: u32) -> Result<(), FuncBuilderError> {
        let op = BrOp::new_unlinked(ctx, relative_depth.into());
        self.fbuilder.push(ctx, op.get_operation())
    }
}
