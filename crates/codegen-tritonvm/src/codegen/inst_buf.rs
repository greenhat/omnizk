use std::collections::HashMap;

use triton_opcodes::instruction::AnInstruction;
use triton_opcodes::instruction::LabelledInstruction;
use triton_opcodes::program::Program;

use crate::TritonOutputFormat;
use crate::TritonTargetConfig;

pub struct InstBuffer<'a> {
    inner: Vec<LabelledInstruction<'a>>,
    comments: HashMap<usize, String>,
}
impl<'a> InstBuffer<'a> {
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
        self.inner.push(LabelledInstruction::Instruction(inst, ""));
    }

    // pub(crate) fn push_with_comment(&mut self, inst: AnInstruction<String>, comment: String) {
    //     self.comments.insert(self.inner.len(), comment);
    //     self.push(inst);
    // }

    pub(crate) fn push_comment_for_next_ins(&mut self, comment: String) {
        self.comments.insert(
            if self.inner.is_empty() {
                0
            } else {
                self.inner.len()
            },
            comment,
        );
    }

    pub(crate) fn append(&mut self, insts: Vec<AnInstruction<String>>) {
        let mut insts = insts
            .into_iter()
            .map(|i| LabelledInstruction::Instruction(i, ""))
            .collect::<Vec<_>>();
        self.inner.append(&mut insts);
    }

    pub(crate) fn push_label(&mut self, label: String) {
        self.inner.push(LabelledInstruction::Label(label, ""));
    }
}
