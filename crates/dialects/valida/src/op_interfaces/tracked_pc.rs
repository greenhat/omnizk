#![allow(clippy::expect_used)]

use ozk_ozk_dialect::attributes::apint_to_u32;
use ozk_ozk_dialect::attributes::u32_attr;
use pliron::context::Context;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::error::CompilerError;
use pliron::op::Op;

use crate::ops::FuncOp;
use crate::ops::ProgramOp;
use crate::types::ProgramCounter;

const ATTR_KEY_TRACK_PC: &str = "track_pc.pc";

pub trait TrackedProgramCounter: Op {
    /// Set program counter for this operation.
    fn set_pc(&self, ctx: &mut Context, pc: ProgramCounter) {
        let attr = u32_attr(ctx, pc.into());
        let mut self_op = self.get_operation().deref_mut(ctx);
        self_op.attributes.insert(ATTR_KEY_TRACK_PC, attr);
    }

    fn get_pc_opt(&self, ctx: &Context) -> Option<ProgramCounter> {
        let self_op = self.get_operation().deref(ctx);
        self_op
            .attributes
            .get(ATTR_KEY_TRACK_PC)
            .and_then(|attr_obj| {
                attr_obj
                    .downcast_ref::<IntegerAttr>()
                    .map(|attr| apint_to_u32(attr.clone().into()).into())
            })
    }

    fn get_pc(&self, ctx: &Context) -> ProgramCounter {
        self.get_pc_opt(ctx)
            .expect("expected program counter to be set by the special pass")
    }

    /// Verify that the operation is valid.
    fn verify(_op: &dyn Op, _ctx: &Context) -> Result<(), CompilerError>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[intertrait::cast_to]
impl TrackedProgramCounter for FuncOp {}

/// An interface for operations with custom pc
pub trait CustomProgramCountChange: Op {
    /// Get the stack depth change for this operation.
    fn get_pc_change(&self, ctx: &Context) -> i32;
}

macro_rules! custom_pc_change {
    ($op:ty, $change:expr) => {
        #[intertrait::cast_to]
        impl CustomProgramCountChange for $op {
            fn get_pc_change(&self, _ctx: &Context) -> i32 {
                $change
            }
        }
    };
}

custom_pc_change!(FuncOp, 0);
custom_pc_change!(ProgramOp, 0);
