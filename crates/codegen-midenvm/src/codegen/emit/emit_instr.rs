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

macro_rules! emit_masm {
    ($op:ty, $builder_method:ident) => {
        #[cast_to]
        impl EmitMasm for $op {
            fn emit_masm(&self, ctx: &Context, builder: &mut MidenAssemblyBuilder) {
                builder.$builder_method();
            }
        }
    };
}

macro_rules! emit_masm_param {
    ($op:ty, $builder_method:ident, $builder_method_param:ident) => {
        #[cast_to]
        impl EmitMasm for $op {
            fn emit_masm(&self, ctx: &Context, builder: &mut MidenAssemblyBuilder) {
                builder.$builder_method(self.$builder_method_param(ctx).into());
            }
        }
    };
}

emit_masm!(AddOp, add);
emit_masm_param!(ConstantOp, push, get_value);
emit_masm_param!(ExecOp, exec, get_callee_sym);
emit_masm_param!(LocLoadOp, loc_load, get_index_as_u32);
