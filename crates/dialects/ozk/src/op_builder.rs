use pliron::context::Context;

use crate::attributes::i32_attr;
use crate::ops::ConstantOp;

pub struct OpBuilder;

impl OpBuilder {
    pub fn new() -> OpBuilder {
        OpBuilder
    }

    pub fn i32const(&mut self, ctx: &mut Context, value: i32) -> ConstantOp {
        let val = i32_attr(ctx, value);
        ConstantOp::new_unlinked(ctx, val)
    }
}

impl Default for OpBuilder {
    fn default() -> Self {
        Self::new()
    }
}
