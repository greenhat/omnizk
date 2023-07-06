use apint::ApInt;
use ozk_ozk_dialect::attributes::u32_attr;
use pliron::attribute;
use pliron::context::Context;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::error::CompilerError;
use pliron::op::Op;

use crate::ops::ConstantOp;

pub trait TrackedStackDepth: Op {
    const ATTR_KEY_STACK_DEPTH: &'static str = "tracked_stack_depth";

    #[allow(clippy::expect_used)]
    fn get_stack_depth(&self, ctx: &Context) -> u32 {
        let self_op = self.get_operation().deref(ctx);
        let value = self_op
            .attributes
            .get(Self::ATTR_KEY_STACK_DEPTH)
            .expect("no attribute found");
        let attr = attribute::clone::<IntegerAttr>(value);
        let apint: ApInt = attr
            .downcast_ref::<IntegerAttr>()
            .expect("IntegerAttr expected")
            .clone()
            .into();
        apint.try_to_u32().expect("expected u32")
    }

    /// Set a name for the symbol defined by this operation.
    fn set_stack_depth(&self, ctx: &mut Context, depth: u32) {
        let name_attr = u32_attr(ctx, depth);
        let mut self_op = self.get_operation().deref_mut(ctx);
        self_op
            .attributes
            .insert(Self::ATTR_KEY_STACK_DEPTH, name_attr);
    }

    fn verify(_op: &dyn Op, _ctx: &Context) -> Result<(), CompilerError>
    where
        Self: Sized,
    {
        Ok(())
    }
}

impl TrackedStackDepth for ConstantOp {}
