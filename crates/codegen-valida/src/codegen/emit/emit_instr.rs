use intertrait::cast_to;
use ozk_valida_dialect::op_interfaces::HasOperands;
use ozk_valida_dialect::ops::AddOp;
use ozk_valida_dialect::ops::ExitOp;
use ozk_valida_dialect::ops::FuncOp;
use ozk_valida_dialect::ops::Imm32Op;
use ozk_valida_dialect::ops::JalOp;
use ozk_valida_dialect::ops::JalvOp;
use ozk_valida_dialect::ops::ProgramOp;
use ozk_valida_dialect::ops::SwOp;
use pliron::context::Context;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;

use crate::codegen::valida_inst_builder::ValidaInstrBuilder;
use crate::emit_op;

pub trait EmitInstr: Op {
    fn emit_instr(&self, ctx: &Context, builder: &mut ValidaInstrBuilder);
}

#[cast_to]
impl EmitInstr for ExitOp {
    fn emit_instr(&self, _ctx: &Context, builder: &mut ValidaInstrBuilder) {
        builder.exit();
    }
}

#[cast_to]
impl EmitInstr for ProgramOp {
    fn emit_instr(&self, ctx: &Context, builder: &mut ValidaInstrBuilder) {
        let mut entry_block_ops = Vec::new();
        for op in self.get_entry_block(ctx).deref(ctx).iter(ctx) {
            entry_block_ops.push(op);
        }
        for op in entry_block_ops {
            emit_op(ctx, op, builder);
        }
        let mut func_ops = Vec::new();
        for func_op in self.get_funcs_block(ctx).deref(ctx).iter(ctx) {
            func_ops.push(func_op);
        }
        for func_op in func_ops {
            emit_op(ctx, func_op, builder);
        }
    }
}

#[cast_to]
impl EmitInstr for FuncOp {
    fn emit_instr(&self, ctx: &Context, builder: &mut ValidaInstrBuilder) {
        let mut ops = Vec::new();
        for func_op in self.get_entry_block(ctx).deref(ctx).iter(ctx) {
            ops.push(func_op);
        }
        for op in ops {
            emit_op(ctx, op, builder);
        }
    }
}

macro_rules! emit_instr {
    ($op:ty, $builder_method:ident) => {
        #[cast_to]
        impl EmitInstr for $op {
            fn emit_instr(&self, ctx: &Context, builder: &mut ValidaInstrBuilder) {
                builder.$builder_method(self.get_operands(ctx));
            }
        }
    };
}

emit_instr!(Imm32Op, imm32);
emit_instr!(AddOp, add);
emit_instr!(JalvOp, jalv);
emit_instr!(JalOp, jal);
emit_instr!(SwOp, sw);