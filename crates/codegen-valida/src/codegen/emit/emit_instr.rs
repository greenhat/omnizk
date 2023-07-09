use ozk_valida_dialect::ops::Imm32Op;
use pliron::context::Context;
use pliron::op::Op;

use crate::codegen::valida_inst_builder::ValidaInstrBuilder;

pub trait EmitInstr: Op {
    fn emit_instr_word(&self, ctx: &Context, builder: &mut ValidaInstrBuilder);
}

impl EmitInstr for Imm32Op {
    fn emit_instr_word(&self, ctx: &Context, builder: &mut ValidaInstrBuilder) {
        builder.imm32(self.get_operands(ctx));
    }
}
