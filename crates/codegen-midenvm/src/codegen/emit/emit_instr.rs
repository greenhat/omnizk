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

emit_masm_param!(ConstantOp, push, get_value);

// #[cast_to]
// impl EmitMasm for ConstantOp {
//     fn emit_masm(&self, ctx: &Context, builder: &mut MidenAssemblyBuilder) {
//         let val = self.get_value(ctx);
//         builder.push(val.into());
//     }
// }

emit_masm!(AddOp, add);

emit_masm_param!(ExecOp, exec, get_callee_sym);

// #[cast_to]
// impl EmitMasm for ExecOp {
//     fn emit_masm(&self, ctx: &Context, builder: &mut MidenAssemblyBuilder) {
//         let callee = self.get_callee_sym(ctx);
//         builder.exec(callee);
//     }
// }

emit_masm_param!(LocLoadOp, loc_load, get_index_as_u32);

// #[cast_to]
// impl EmitMasm for LocLoadOp {
//     fn emit_masm(&self, ctx: &Context, builder: &mut MidenAssemblyBuilder) {
//         let index = self.get_index_as_u32(ctx);
//         builder.loc_load(index);
//     }
// }
