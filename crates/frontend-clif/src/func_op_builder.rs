use ozk_ozk_dialect::attributes::i32_attr;
use ozk_ozk_dialect::ops::ConstantOp;
use pliron::context::Context;
use pliron::op::Op;

use crate::func_builder::FuncBuilder;

pub struct FuncOpBuilder<'a> {
    fbuilder: &'a mut FuncBuilder,
}

impl<'a> FuncOpBuilder<'a> {
    pub fn new(fbuilder: &'a mut FuncBuilder) -> FuncOpBuilder<'a> {
        FuncOpBuilder { fbuilder }
    }

    pub fn i32const(&mut self, ctx: &mut Context, value: i32) {
        let attr = i32_attr(ctx, value);
        let op = ConstantOp::new_unlinked(ctx, attr).get_operation();
        self.fbuilder.append(ctx, op);
    }
}
