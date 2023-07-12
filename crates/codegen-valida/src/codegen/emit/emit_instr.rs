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
use pliron::op::op_cast;
use pliron::op::Op;
use pliron::with_context::AttachContext;

use crate::codegen::valida_inst_builder::ValidaInstrBuilder;
use crate::emit_op;

pub trait EmitInstr: Op {
    fn emit_instr(&self, ctx: &Context, builder: &mut ValidaInstrBuilder);
}

#[intertrait::cast_to]
impl EmitInstr for ExitOp {
    fn emit_instr(&self, _ctx: &Context, builder: &mut ValidaInstrBuilder) {
        builder.exit();
    }
}

macro_rules! emit_instr {
    ($op:ty, $builder_method:ident) => {
        #[intertrait::cast_to]
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

#[intertrait::cast_to]
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

        /* Moved to wasm module lowering pass

        // call the main function
        let size_of_current_stack = 16;
        let call_frame_size = 12;
        // call label is a pseudo op which consist of:
        // imm32 (-b+8)(fp), 0, 0, 0, b(fp)
        // jal -b(fp), label, -b(fp)
        // , where b is the size of the current stack frame plus the call frame size for instantiating a call to label
        let b = size_of_current_stack + call_frame_size;
        let main_func_pc = 4;
        // pc == 0
        builder.imm32(Operands::from_i32(-b + 8, 0, 0, 0, b));
        // pc == 1
        builder.jal(Operands::from_i32(-b, main_func_pc, -b, 0, 0));
        // pc == 2
        builder.sw(Operands::from_i32(0, 4, -24, 0, 0));
        // pc == 3
        builder.exit();
        // pc == 4 the start of the next(main) function
        */
        for func_op in func_ops {
            emit_op(ctx, func_op, builder);
        }
    }
}

#[intertrait::cast_to]
impl EmitInstr for FuncOp {
    fn emit_instr(&self, ctx: &Context, builder: &mut ValidaInstrBuilder) {
        let mut ops = Vec::new();
        for func_op in self.get_entry_block(ctx).deref(ctx).iter(ctx) {
            ops.push(func_op);
        }
        for op in ops {
            let deref = op.deref(ctx).get_op(ctx);
            #[allow(clippy::panic)]
            let emitable_op = op_cast::<dyn EmitInstr>(deref.as_ref())
                .unwrap_or_else(|| panic!("missing EmitInstr impl for {}", op.with_ctx(ctx)));
            emitable_op.emit_instr(ctx, builder);
        }
    }
}
