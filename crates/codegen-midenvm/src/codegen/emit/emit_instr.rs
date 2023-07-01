use intertrait::cast_to;
use ozk_miden_dialect::ops::AddOp;
use ozk_miden_dialect::ops::ConstantOp;
use ozk_miden_dialect::ops::ExecOp;
use ozk_miden_dialect::ops::LocLoadOp;
use pliron::context::Context;
use pliron::op::Op;

use crate::MidenAssemblyBuilder;

pub trait EmitMasm: Op {
    fn emit_masm(&self, ctx: &Context, builder: &mut MidenAssemblyBuilder);
}

#[cast_to]
impl EmitMasm for ConstantOp {
    fn emit_masm(&self, ctx: &Context, builder: &mut MidenAssemblyBuilder) {
        let val = self.get_value(ctx);
        builder.push(val.into());
    }
}

#[cast_to]
impl EmitMasm for AddOp {
    fn emit_masm(&self, ctx: &Context, builder: &mut MidenAssemblyBuilder) {
        builder.add();
    }
}

#[cast_to]
impl EmitMasm for ExecOp {
    fn emit_masm(&self, ctx: &Context, builder: &mut MidenAssemblyBuilder) {
        let callee = self.get_callee_sym(ctx);
        builder.exec(callee);
    }
}

#[cast_to]
impl EmitMasm for LocLoadOp {
    fn emit_masm(&self, ctx: &Context, builder: &mut MidenAssemblyBuilder) {
        let index = self.get_index_as_u32(ctx);
        builder.loc_load(index);
    }
}
