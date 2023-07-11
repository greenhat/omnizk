use pliron::context::Context;
use pliron::context::Ptr;
use pliron::op::op_cast;
use pliron::operation::Operation;
use pliron::with_context::AttachContext;
use thiserror::Error;

use self::emit_instr::EmitInstr;

use super::valida_inst_builder::ValidaInstrBuilder;

mod emit_instr;

#[derive(Debug, Error)]
pub enum EmitError {}

pub fn emit_op(
    ctx: &Context,
    op: Ptr<Operation>,
    b: &mut ValidaInstrBuilder,
) -> Result<(), EmitError> {
    #[allow(clippy::panic)] // all ops should be emitable
    if let Some(emitable_op) = op_cast::<dyn EmitInstr>(op.deref(ctx).get_op(ctx).as_ref()) {
        emitable_op.emit_instr(ctx, b);
    } else {
        panic!("cannot emit op: {}", op.with_ctx(ctx));
    }
    Ok(())
}
