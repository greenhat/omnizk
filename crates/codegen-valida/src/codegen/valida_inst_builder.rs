use ozk_valida_dialect::types::Operands;
use valida_alu_u32::add::Add32Instruction;
use valida_basic::BasicMachine;
use valida_cpu::Imm32Instruction;
use valida_cpu::JalInstruction;
use valida_cpu::JalvInstruction;
use valida_cpu::Store32Instruction;
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

    // pub fn pretty_print(&self) -> String {
    //     let mut sink = String::new();
    //     for instr in &self.sink {
    //         sink.push_str(&format!("{:?}\n", instr.opcode));
    //     }
    //     sink
    // }

    /// Emit an `exit` instruction halting the VM
    pub fn exit(&mut self) {
        self.sink.push(InstructionWord {
            opcode: 0,
            operands: valida_machine::Operands::default(),
        });
    }
}

macro_rules! impl_op {
    ($op:ident, $valida_op:ty) => {
        impl ValidaInstrBuilder {
            pub fn $op(&mut self, operands: Operands) {
                self.sink.push(InstructionWord {
                    opcode: <$valida_op as Instruction<BasicMachine>>::OPCODE,
                    operands: operands.into(),
                });
            }
        }
    };
}

impl_op!(add, Add32Instruction);
impl_op!(imm32, Imm32Instruction);
impl_op!(jalv, JalvInstruction);
impl_op!(jal, JalInstruction);
impl_op!(sw, Store32Instruction);
