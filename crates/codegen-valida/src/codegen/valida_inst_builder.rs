use ozk_valida_dialect::types::Operands;
use valida_basic::BasicMachine;
use valida_cpu::Imm32Instruction;
use valida_machine::Instruction;
use valida_machine::InstructionWord;

pub struct ValidaInstrBuilder {
    sink: Vec<InstructionWord<i32>>,
}

impl ValidaInstrBuilder {
    pub fn new() -> Self {
        Self { sink: Vec::new() }
    }

    pub fn build(self) -> Vec<InstructionWord<i32>> {
        self.sink
    }

    pub fn imm32(&mut self, operands: Operands) {
        self.sink.push(InstructionWord {
            opcode: <Imm32Instruction as Instruction<BasicMachine>>::OPCODE,
            operands: operands.into(),
        });
    }
}
