use triton_vm::instruction::AnInstruction;
use triton_vm::instruction::LabelledInstruction;

use crate::TritonOutputFormat;
use crate::TritonTargetConfig;

pub struct InstBuffer {
    inner: Vec<LabelledInstruction>,
}
impl InstBuffer {
    pub(crate) fn new(config: &TritonTargetConfig) -> Self {
        match config.output_format {
            TritonOutputFormat::Binary => todo!(),
            TritonOutputFormat::Source => Self { inner: Vec::new() },
        }
    }

    pub(crate) fn pretty_print(&self) -> String {
        self.inner
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub(crate) fn push(&mut self, inst: AnInstruction<String>) {
        self.inner.push(LabelledInstruction::Instruction(inst));
    }

    pub(crate) fn push_label(&mut self, label: String) {
        self.inner.push(LabelledInstruction::Label(label));
    }
}
