use ozk_miden_dialect::ops::AddOp;
use ozk_miden_dialect::ops::ConstantOp;
use ozk_miden_dialect::ops::ExecOp;
use ozk_miden_dialect::ops::LocLoadOp;
use pliron::context::Context;
use pliron::op::Op;

use crate::EmitError;
use crate::MidenAssemblyBuilder;

pub trait EmitMasm: Op {
    fn emit_masm(&self, ctx: &Context, builder: &MidenAssemblyBuilder) -> Result<(), EmitError>;
}

impl EmitMasm for ConstantOp {
    fn emit_masm(&self, ctx: &Context, builder: &MidenAssemblyBuilder) -> Result<(), EmitError> {
        let val = self.get_value(ctx);
        builder.push(val.into());
        Ok(())
    }
}

impl EmitMasm for AddOp {
    fn emit_masm(&self, ctx: &Context, builder: &MidenAssemblyBuilder) -> Result<(), EmitError> {
        builder.add();
        Ok(())
    }
}

impl EmitMasm for ExecOp {
    fn emit_masm(&self, ctx: &Context, builder: &MidenAssemblyBuilder) -> Result<(), EmitError> {
        let callee = self.get_callee_sym(ctx);
        builder.exec(callee);
        Ok(())
    }
}

impl EmitMasm for LocLoadOp {
    fn emit_masm(&self, ctx: &Context, builder: &MidenAssemblyBuilder) -> Result<(), EmitError> {
        let local = self.get_index(ctx);
        builder.loc_load(local);
        Ok(())
    }
}
