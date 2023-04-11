use std::collections::HashMap;

use triton_opcodes::instruction::AnInstruction;
use triton_opcodes::instruction::LabelledInstruction;
use triton_opcodes::program::Program;

use crate::TritonOutputFormat;
use crate::TritonTargetConfig;

pub struct InstBuffer {
    inner: Vec<LabelledInstruction>,
    comments: HashMap<usize, String>,
}
impl InstBuffer {
    pub(crate) fn new(config: &TritonTargetConfig) -> Self {
        match config.output_format {
            TritonOutputFormat::Binary => todo!(),
            TritonOutputFormat::Source => Self {
                inner: Vec::new(),
                comments: HashMap::new(),
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn program(&self) -> Program {
        Program::new(&self.inner)
    }

    pub(crate) fn pretty_print(&self) -> String {
        self.inner
            .iter()
            .enumerate()
            .map(|(idx, ins)| match self.comments.get(&idx) {
                Some(note) => format!("{} // {}", ins, note),
                None => format!("{}", ins),
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub(crate) fn push(&mut self, inst: AnInstruction<String>) {
        self.inner.push(LabelledInstruction::Instruction(inst));
    }

    pub(crate) fn append(&mut self, insts: Vec<AnInstruction<String>>) {
        let mut insts = insts
            .into_iter()
            .map(LabelledInstruction::Instruction)
            .collect::<Vec<_>>();
        self.inner.append(&mut insts);
    }

    pub(crate) fn push_label(&mut self, label: String) {
        self.inner.push(LabelledInstruction::Label(label));
    }
}
