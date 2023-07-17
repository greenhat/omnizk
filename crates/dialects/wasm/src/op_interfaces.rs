//! This module defines the interfaces for the operations in the wasm dialect.

#![allow(clippy::expect_used)]

use apint::ApInt;
use ozk_ozk_dialect::attributes::u32_attr;
use pliron::attribute;
use pliron::context::Context;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::error::CompilerError;
use pliron::op::Op;

use crate::ops::AddOp;
use crate::ops::ConstantOp;
use crate::ops::LocalGetOp;
use crate::ops::LocalSetOp;
use crate::ops::ReturnOp;
use crate::types::StackDepth;

/// The attribute key for the stack depth.
const ATTR_KEY_STACK_DEPTH: &str = "tracked_stack_depth";

/// An interface for operations that have a stack depth calculated.
pub trait TrackedStackDepth: Op {
    /// Get the stack depth before this operation.
    fn get_stack_depth(&self, ctx: &Context) -> StackDepth {
        let self_op = self.get_operation().deref(ctx);
        let value = self_op
            .attributes
            .get(ATTR_KEY_STACK_DEPTH)
            .expect("no stack depth attribute found, expected it to be set by the special pass");
        let attr = attribute::clone::<IntegerAttr>(value);
        let apint: ApInt = attr
            .downcast_ref::<IntegerAttr>()
            .expect("IntegerAttr expected")
            .clone()
            .into();
        apint.try_to_u32().expect("expected u32").into()
    }

    /// Set a name for the symbol defined by this operation.
    fn set_stack_depth(&self, ctx: &mut Context, depth: StackDepth) {
        let depth_attr = u32_attr(ctx, depth.into());
        let mut self_op = self.get_operation().deref_mut(ctx);
        self_op.attributes.insert(ATTR_KEY_STACK_DEPTH, depth_attr);
    }

    /// Verify that the operation is valid.
    fn verify(_op: &dyn Op, _ctx: &Context) -> Result<(), CompilerError>
    where
        Self: Sized,
    {
        Ok(())
    }
}

/// An interface for operations to get a stack depth change.
pub trait StackDepthChange: Op {
    /// Get the stack depth change for this operation.
    fn get_stack_depth_change(&self, ctx: &Context) -> i32;
}

#[intertrait::cast_to]
impl TrackedStackDepth for ozk_ozk_dialect::ops::CallOp {}

impl StackDepthChange for ozk_ozk_dialect::ops::CallOp {
    fn get_stack_depth_change(&self, ctx: &Context) -> i32 {
        let func_type = self.get_func_type(ctx);
        -(func_type.get_inputs().len() as i32) + func_type.get_results().len() as i32
    }
}

macro_rules! stack_depth_change {
    ($op:ty, $change:expr) => {
        #[intertrait::cast_to]
        impl TrackedStackDepth for $op {}

        #[intertrait::cast_to]
        impl StackDepthChange for $op {
            fn get_stack_depth_change(&self, _ctx: &Context) -> i32 {
                $change
            }
        }
    };
}

stack_depth_change!(ConstantOp, 1);
stack_depth_change!(AddOp, -1);
stack_depth_change!(ReturnOp, 0);
stack_depth_change!(LocalGetOp, 1);
stack_depth_change!(LocalSetOp, -1);
