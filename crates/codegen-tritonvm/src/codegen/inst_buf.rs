use triton_vm::instruction::Instruction;

use crate::TritonOutputFormat;
use crate::TritonTargetConfig;

pub struct InstBuffer {
    inner: Vec<Instruction>,
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

    pub(crate) fn push(&mut self, inst: Instruction) {
        self.inner.push(inst);
    }
}
