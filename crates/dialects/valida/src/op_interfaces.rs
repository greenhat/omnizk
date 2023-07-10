#![allow(clippy::expect_used)]

use ozk_ozk_dialect::attributes::i32_attr;
use pliron::context::Context;
use pliron::dialects::builtin::attributes::VecAttr;
use pliron::error::CompilerError;
use pliron::op::Op;

use crate::types::Operands;

/// The attribute key for the operands.
const ATTR_KEY_HAS_OPERANDS: &str = "has_operands.operands";

pub trait HasOperands: Op {
    /// Set the operands for this operation.
    fn set_operands(&self, ctx: &mut Context, operands: Operands) {
        let a = i32_attr(ctx, operands.a().as_i32());
        let b = i32_attr(ctx, operands.b().as_i32());
        let c = i32_attr(ctx, operands.c().as_i32());
        let d = i32_attr(ctx, operands.d().as_i32());
        let e = i32_attr(ctx, operands.e().as_i32());
        let attr = VecAttr::create(vec![a, b, c, d, e]);
        let mut self_op = self.get_operation().deref_mut(ctx);
        self_op.attributes.insert(ATTR_KEY_HAS_OPERANDS, attr);
    }

    /// Get the operands for this operation.
    fn get_operands(&self, ctx: &Context) -> Operands {
        let self_op = self.get_operation().deref(ctx);
        let attr_obj = self_op
            .attributes
            .get(ATTR_KEY_HAS_OPERANDS)
            .expect("no operands attribute found");
        let vec_attr = attr_obj
            .downcast_ref::<VecAttr>()
            .expect("VecAttr expected");
        Operands::try_from(vec_attr).expect("expected Operands")
    }

    /// Verify that the operation is valid.
    fn verify(_op: &dyn Op, _ctx: &Context) -> Result<(), CompilerError>
    where
        Self: Sized,
    {
        todo!()
    }
}
